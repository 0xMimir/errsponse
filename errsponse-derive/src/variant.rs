#![allow(unused)]

use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse::Parse, parse_macro_input, Attribute, Error, Fields, Token, Variant};

pub struct DataVariant {
    name: Ident,
    style: Fields,
    attributes: VariantAttribute,
}

impl DataVariant {
    pub fn cause(&self) -> TokenStream {
        let mut head = self.head();
        let mut return_value = quote!(errsponse::serde_json::Value::Null);

        if let Some(cause) = self.attributes.cause.as_ref() {
            return_value = quote!(errsponse::serde_json::Value::String(format!(#cause)))
        } else if self.attributes.json {
            return_value = quote!(errsponse::serde_json::to_value(&value).unwrap_or_default())
        } else if self.attributes.nested {
            return_value = quote!(value.cause())
        }

        quote!(#head => #return_value)
    }

    pub fn status_code(&self) -> TokenStream {
        let head = self.head();
        let status = &self.attributes.status_code;

        match self.attributes.nested {
            true => quote!(#head => value.status_code()),
            false => quote!(#head => errsponse::http::StatusCode::#status),
        }
    }

    fn head(&self) -> TokenStream {
        let name = &self.name;
        let brackets = match &self.style {
            Fields::Named(fields) => {
                let fields = fields.named.iter().filter_map(|field| field.ident.as_ref());
                quote!({#(#fields)*,})
            }
            Fields::Unnamed(_) => quote!((value)),
            Fields::Unit => quote!(),
        };

        quote!(Self::#name #brackets )
    }
}

pub struct VariantAttribute {
    status_code: Ident,
    json: bool,
    nested: bool,
    cause: Option<Literal>,
}

impl Default for VariantAttribute {
    fn default() -> Self {
        Self {
            status_code: Ident::new("INTERNAL_SERVER_ERROR", Span::call_site()),
            json: false,
            nested: false,
            cause: None,
        }
    }
}

impl TryFrom<Variant> for DataVariant {
    type Error = syn::Error;

    fn try_from(value: Variant) -> Result<Self, syn::Error> {
        let attributes = match value
            .attrs
            .into_iter()
            .find(VariantAttribute::has_attribute)
        {
            Some(attributes) => match attributes.meta {
                syn::Meta::Path(_) => VariantAttribute::default(),
                syn::Meta::List(meta) => syn::parse(meta.tokens.into())?,
                syn::Meta::NameValue(_) => {
                    return Err(Error::new(Span::call_site(), "Invalid value passed"))
                }
            },

            None => VariantAttribute::default(),
        };

        Ok(Self {
            name: value.ident,
            style: value.fields,
            attributes,
        })
    }
}

impl VariantAttribute {
    fn has_attribute(attributes: &Attribute) -> bool {
        attributes
            .meta
            .path()
            .segments
            .iter()
            .any(|segment| segment.ident == "response")
    }

    fn recurse(&mut self, stream: syn::parse::ParseStream) -> syn::Result<()> {
        if stream.is_empty() {
            return Ok(());
        }

        let attribute = stream.parse::<Ident>()?;

        match attribute.to_string().as_str() {
            "json" => self.json = true,
            "status" => {
                stream.parse::<Token![=]>()?;
                self.status_code = stream.parse()?;
            }
            "cause" => {
                stream.parse::<Token![=]>()?;
                self.cause = Some(stream.parse()?);
            }
            "nested" => self.nested = true,
            _ => {
                return Err(Error::new(
                    stream.span(),
                    format!("Invalid attribute: `{}`", attribute),
                ))
            }
        }

        if stream.peek(Token![,]) {
            stream.parse::<Token![,]>()?;
        }

        self.recurse(stream)
    }
}

impl Parse for VariantAttribute {
    fn parse(stream: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut attribute = Self::default();
        attribute.recurse(stream)?;
        Ok(attribute)
    }
}
