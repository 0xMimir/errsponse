use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{parse::Parse, Attribute};

pub fn should_derive_default_response(attrs: &[Attribute]) -> syn::Result<bool> {
    let attribute = match attrs.iter().find(|meta| {
        meta.meta
            .path()
            .segments
            .iter()
            .any(|segment| segment.ident == "response")
    }) {
        Some(attribute) => attribute,
        None => return Ok(false),
    };

    attribute.parse_args::<DeriveDefaultResponse>().map(|s| s.0)
}

struct DeriveDefaultResponse(bool);

impl Parse for DeriveDefaultResponse {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let value = input.parse::<Ident>()?;
        Ok(Self(value == "default"))
    }
}

pub fn derive_default_response() -> TokenStream {
    quote! {
    #[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
    pub struct ErrorResponse {
        pub status: u16,
        pub message: String,
        pub cause: Value,
        pub time: chrono::NaiveDateTime,
    }

    impl<T> From<T> for ErrorResponse
    where
        T: ImplErrorResponse,
    {
        fn from(value: T) -> Self {
            let status = value.status_code();

            let message = status
                .canonical_reason()
                .map(|cause| cause.to_owned())
                .unwrap_or_default();

            let cause = value.cause();
            let time = value.time();

            ErrorResponse {
                status: status.as_u16(),
                message,
                cause,
                time,
            }
        }
    }
    }
}
