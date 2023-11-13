use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, DataEnum, DeriveInput, Error, Token};

use crate::{
    default_response::{derive_default_response, should_derive_default_response},
    variant::DataVariant,
};

pub fn implement_error_response(input: DeriveInput) -> TokenStream {
    match &input.data {
        syn::Data::Enum(data) => derive_enum(&input, data),
        _ => Error::new_spanned(input, "ErrorResponse can only be derived on enums")
            .to_compile_error(),
    }
}

pub fn derive_enum(input: &DeriveInput, data: &DataEnum) -> TokenStream {
    let enum_name = &input.ident;

    let derive_default = match should_derive_default_response(input.attrs.as_ref()) {
        Ok(value) => value,
        Err(error) => return error.to_compile_error(),
    };

    let mut status_code_statements: Punctuated<TokenStream, Token![,]> = Punctuated::new();
    let mut cause_statements: Punctuated<TokenStream, Token![,]> = Punctuated::new();

    for variant in &data.variants {
        let data_variant = match DataVariant::try_from(variant.clone()) {
            Ok(variant) => variant,
            Err(error) => return error.to_compile_error(),
        };

        status_code_statements.push(data_variant.status_code());
        cause_statements.push(data_variant.cause());
    }

    let default_response = derive_default.then(derive_default_response);

    quote!(
        impl errsponse::ImplErrorResponse for #enum_name{
            fn status_code(&self) -> errsponse::http::StatusCode {
                match self{
                    #status_code_statements
                }
            }

            fn cause(&self) -> errsponse::serde_json::Value {
                match self{
                    #cause_statements
                }
            }
        }

        #default_response
    )
}
