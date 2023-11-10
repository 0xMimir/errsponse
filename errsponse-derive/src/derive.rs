use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, DeriveInput, Error};

use crate::variant::DataVariant;

pub fn implement_error_response(input: DeriveInput) -> TokenStream {
    match &input.data {
        syn::Data::Enum(data) => derive_enum(&input, data),
        _ => Error::new_spanned(input, "ErrorResponse can only be derived on enums")
            .to_compile_error(),
    }
}

pub fn derive_enum(input: &DeriveInput, data: &DataEnum) -> TokenStream {
    let enum_name = &input.ident;

    let mut status_code_statements = vec![];
    let mut cause_statments = vec![];

    for variant in &data.variants {
        let data_variant = match DataVariant::try_from(variant.clone()) {
            Ok(variant) => variant,
            Err(error) => return error,
        };

        status_code_statements.push(data_variant.status_code());
        cause_statments.push(data_variant.cause());
    }

    let f = quote!(
        impl errsponse::ImplErrorResponse for #enum_name{
            fn status_code(&self) -> errsponse::http::StatusCode {
                match self{
                    #(#status_code_statements)*
                }
            }

            fn cause(&self) -> errsponse::serde_json::Value {
                match self{
                    #(#cause_statments)*
                }
            }
        }
    );
    f
}
