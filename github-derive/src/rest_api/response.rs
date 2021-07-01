use proc_macro2::TokenStream;
use quote::quote;

use crate::rest_api::parse::{DeserAttr, Response};

pub(crate) fn expand_response(res: &Response) -> syn::Result<TokenStream> {
    let single_attr = res
        .attrs
        .iter()
        .filter_map(|attr| {
            attr.path
                .is_ident("github")
                .then(|| attr.parse_args::<DeserAttr>().ok())
                .flatten()
        })
        .collect::<Vec<_>>();
    if single_attr.len() > 1 {
        return Err(syn::Error::new_spanned(
            &res.resp_kw,
            "only one attribute can be applied to the response",
        ));
    }
    let single_attr = single_attr.first();
    let (impl_deser, derive_deser) = if let Some(DeserAttr::With(path)) = single_attr {
        (
            quote! {
                impl<'de> ::serde::de::Deserialize<'de> for Response {
                    fn deserialize<D>(deser: D) -> Result<Response, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        #path (deser)
                    }
                }
            },
            TokenStream::new(),
        )
    } else {
        (TokenStream::new(), quote! { ::serde::Deserialize })
    };

    let attr = res.attrs.iter().filter(|attr| !attr.path.is_ident("github"));
    let field = res.res_fields.iter();

    let response = quote! {
        #[derive(Clone, Debug, #derive_deser)]
        #( #attr )*
        pub struct Response { #( #field ),* }
    };

    let return_response = if res.res_fields.is_empty() {
        quote! { Ok(Response {}) }
    } else if let Some(DeserAttr::ForwardToBody(field)) = single_attr {
        quote! {
            Ok(Response { #field: resp.text().await? })
        }
    // TODO: remove possibly ??
    } else if let Some(DeserAttr::ForwardToBodyWith(field, call)) = single_attr {
        quote! {
            Ok(Response { #field: #call(resp).await? })
        }
    } else {
        quote! {
            let json = resp.text().await?;

            println!("{}", json);

            let jd = &mut ::serde_json::Deserializer::from_str(
                &json
            );
            ::serde_path_to_error::deserialize(jd).map_err(Into::into)
            // ::serde_json::from_str(
            //     &resp.text().await?
            // ).map_err(Into::into)
        }
    };

    let impl_from_reqwest = quote! {
        #[::async_trait::async_trait]
        impl ::gitty_hub::GithubResponse for Response {
            async fn from_response(resp: reqwest::Response) -> Result<Response, ::gitty_hub::Error> {
                ::gitty_hub::from_status(resp.status())?;

                #return_response
            }
        }
    };

    Ok(quote! {
        #response
        #impl_deser
        #impl_from_reqwest
    })
}
