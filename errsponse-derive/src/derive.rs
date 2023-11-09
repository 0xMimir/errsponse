use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, DeriveInput, Error};

pub fn implement_error_response(input: DeriveInput) -> TokenStream {
    match &input.data {
        syn::Data::Enum(data) => derive_enum(&input, data),
        _ => Error::new_spanned(input, "ErrorResponse can only be derived on enums")
            .to_compile_error(),
    }
}

pub fn derive_enum(input: &DeriveInput, _data: &DataEnum) -> TokenStream {
    let enum_name = &input.ident;

    quote!(
        impl errsponse::ImplErrorResponse for #enum_name{
            fn status_code(&self) -> errsponse::http::StatusCode {
                errsponse::http::StatusCode::INTERNAL_SERVER_ERROR
            }

            fn cause(&self) -> errsponse::serde_json::Value {
                errsponse::serde_json::Value::Null
            }
        }
    )
}
