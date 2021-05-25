use pm2::Span;
use proc_macro::TokenStream;
use proc_macro2 as pm2;
use quote::{quote, ToTokens};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    visit::Visit,
    Attribute, DeriveInput, Field, Ident, Lifetime, Lit, LitStr, Token, Type,
};

mod incoming;
mod rest_api;

#[proc_macro]
pub fn github_rest_api(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as rest_api::GithubInput);
    rest_api::expand(input).unwrap_or_else(syn::Error::into_compile_error).into()
}

#[proc_macro_derive(Incoming, attributes(no_deserialize))]
pub fn incoming_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    incoming::expand(input).unwrap_or_else(syn::Error::into_compile_error).into()
}
