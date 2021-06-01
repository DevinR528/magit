use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, ItemEnum};

mod incoming;
mod rest_api;
mod string_enum;

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

/// Shorthand for the derives `AsRefStr`, `FromString`, `DisplayAsRefStr`,
/// `SerializeAsRefStr` and `DeserializeFromCowStr`.
#[proc_macro_derive(StringEnum, attributes(github_enum))]
pub fn derive_string_enum(input: TokenStream) -> TokenStream {
    fn expand_all(input: ItemEnum) -> syn::Result<proc_macro2::TokenStream> {
        let as_ref_str_impl = string_enum::expand_enum_as_ref_str(&input)?;
        let from_string_impl = string_enum::expand_enum_from_string(&input)?;
        let display_impl = string_enum::expand_display_as_ref_str(&input.ident)?;
        let serialize_impl = string_enum::expand_serialize_as_ref_str(&input.ident)?;
        let deserialize_impl =
            string_enum::expand_deserialize_from_cow_str(&input.ident)?;

        Ok(quote::quote! {
            #as_ref_str_impl
            #from_string_impl
            #display_impl
            #serialize_impl
            #deserialize_impl
        })
    }

    let input = parse_macro_input!(input as ItemEnum);
    expand_all(input).unwrap_or_else(syn::Error::into_compile_error).into()
}
