use proc_macro2::TokenStream;
use quote::quote;

mod parse;
mod request;
mod response;

pub(crate) use parse::GithubInput;

pub(crate) fn expand(api: GithubInput) -> syn::Result<TokenStream> {
    let request = request::expand_request(&api.request, &api.metadata)?;
    let response = response::expand_response(&api.response)?;
    Ok(quote! {
        #request
        #response
    })
}
