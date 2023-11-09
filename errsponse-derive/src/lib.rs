use proc_macro::TokenStream;
use syn::parse;

mod derive;

#[proc_macro_derive(ErrorResponse, attributes(response))]
pub fn derive_response(input: TokenStream) -> TokenStream {
    let input = match parse(input) {
        Ok(tokens) => tokens,
        Err(error) => return error.to_compile_error().into(),
    };

    derive::implement_error_response(input).into()
}
