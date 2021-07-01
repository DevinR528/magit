use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, Type, TypePath};

use crate::rest_api::parse::{check_for_lifetimes, AttrArg, Metadata, Request};

struct RequestField {
    attr: Option<AttrArg>,
    name: Ident,
    ty: Type,
}

fn expand_metadata(meta: &Metadata) -> syn::Result<TokenStream> {
    let Metadata { description, method, path, name, authentication, .. } = meta;

    let meta = format!("Metadata for the {} endpoint.", name.value());
    Ok(quote! {
        #[doc = #meta]
        ///
        #[doc = #description]
        const METADATA: ::gitty_hub::MetaData = ::gitty_hub::MetaData {
            description: #description,
            method: ::reqwest::Method::#method,
            path: #path,
            name: #name,
            authentication: #authentication,
        };
    })
}

pub(crate) fn expand_request(req: &Request, meta: &Metadata) -> syn::Result<TokenStream> {
    let metadata = expand_metadata(meta)?;

    let mut has_all_lifetimes = false;
    let mut fields = vec![];
    for f in &req.req_fields {
        if check_for_lifetimes(&f.ty)? {
            has_all_lifetimes = true;
        }
        let mut single = 0;
        let attrs = f
            .attrs
            .iter()
            .filter_map(|attr| {
                attr.path.is_ident("github").then(|| {
                    single += 1;
                    attr.parse_args::<AttrArg>()
                })
            })
            .collect::<syn::Result<Vec<_>>>()?;
        if attrs.len() > 1 {
            return Err(syn::Error::new_spanned(f, "only 1 github attribute allowed"));
        }

        fields.push(RequestField {
            attr: attrs.into_iter().next(),
            name: f.ident.clone().ok_or_else(|| {
                syn::Error::new_spanned(f, "only 1 github attribute allowed")
            })?,
            ty: f.ty.clone(),
        });
    }

    let accept_header = if fields
        .iter()
        .any(|f|
            f.name == "accept"
                && matches!(
                    &f.ty,
                    Type::Path(TypePath { path, .. }) if path.segments.last().map_or(false, |p| p.ident == "Option")
                )
        )
    {
        quote! {
            let request = if let Some(accept) = self.accept.as_ref() {
                request.header(::reqwest::header::ACCEPT, accept.to_string())
            } else {
                request
            };
        }
    } else {
        quote! {
            let request = request.header(::reqwest::header::ACCEPT, self.accept.to_string());
        }
    };

    let mut fmt_string = meta.path.value();
    let mut fmt_args = vec![];
    for path in fields.iter().filter(|f| matches!(f.attr, Some(AttrArg::Path))) {
        if let Some(start_of_segment) = fmt_string.find(':') {
            // ':' should only ever appear at the start of a segment
            assert_eq!(&fmt_string[start_of_segment - 1..start_of_segment], "/");

            let end_of_segment = match fmt_string[start_of_segment..].find('/') {
                Some(rel_pos) => start_of_segment + rel_pos,
                None => fmt_string.len(),
            };

            let path_var = Ident::new(
                &fmt_string[start_of_segment + 1..end_of_segment],
                Span::call_site(),
            );

            fmt_args.push(quote! {
                ::percent_encoding::percent_decode_str(&self.#path_var.to_string()).decode_utf8_lossy().to_string()
            });
            fmt_string.replace_range(start_of_segment..end_of_segment, "{}");
        } else {
            return Err(syn::Error::new_spanned(
                &path.name,
                "fields marked with `path` must have path segemnts ie `:ident`",
            ));
        }
    }
    let path_str = if fmt_args.is_empty() {
        quote! { Self::METADATA.path.to_owned() }
    } else {
        quote! {
            format_args!(#fmt_string, #(#fmt_args),*)
        }
    };

    let mut query_field = vec![];
    let mut init_field = vec![];
    let mut has_query_lifetime = false;
    for (field, orig) in fields
        .iter()
        .zip(req.req_fields.iter())
        .filter(|(f, _)| matches!(f.attr, Some(AttrArg::Query)))
    {
        if check_for_lifetimes(&field.ty)? {
            has_query_lifetime = true;
        }

        let attr = orig.attrs.iter().filter(|attr| !attr.path.is_ident("github"));
        let name = &field.name;
        let ty = &field.ty;
        query_field.push(quote! {
            #( #attr )* #name: #ty,
        });
        init_field.push(quote! {
            #name: self.#name,
        });
    }
    let query = if query_field.is_empty() {
        TokenStream::new()
    } else {
        let lifetime = if has_query_lifetime {
            quote! { <'a> }
        } else {
            TokenStream::new()
        };
        quote! {
            #[derive(Clone, Debug, ::serde::Serialize)]
            pub struct RequestQuery #lifetime { #( #query_field )* }

            let query = RequestQuery {
                #( #init_field )*
            };

            let request = request.query(&query);
        }
    };

    let mut body_field = vec![];
    let mut init_field = vec![];
    let mut has_body_lifetime = false;
    for (field, orig) in fields
        .iter()
        .zip(req.req_fields.iter())
        .filter(|(f, _)| matches!(f.attr, Some(AttrArg::Body)))
    {
        if check_for_lifetimes(&field.ty)? {
            has_body_lifetime = true;
        }

        let attr = orig.attrs.iter().filter(|attr| !attr.path.is_ident("github"));
        let name = &field.name;
        let ty = &field.ty;
        body_field.push(quote! {
            #( #attr )* #name: #ty,
        });
        init_field.push(quote! {
            #name: self.#name,
        });
    }
    let body = if body_field.is_empty() {
        TokenStream::new()
    } else {
        let lifetime = if has_body_lifetime {
            quote! { <'a> }
        } else {
            TokenStream::new()
        };
        quote! {
            #[derive(Clone, Debug, ::serde::Serialize)]
            pub struct RequestBody #lifetime { #( #body_field )* }

            let body = RequestBody {
                #( #init_field )*
            };

            let json = ::serde_json::to_string(&body)?;

            /// TODO
            println!("{}", json);

            let request = request
                .header(::reqwest::header::CONTENT_LENGTH, json.len())
                .body(json);
        }
    };

    let to_reqwest = quote! {
        impl<'a> ::gitty_hub::GithubRequest for Request<'a> {
            #metadata
            type Response = Response;
            fn to_request(
                self,
                github: &::gitty_hub::GithubClient
            ) -> Result<::reqwest::Request, ::gitty_hub::Error> {
                let request = github.request_builder(Self::METADATA.method, &::std::format!(
                    "{}{}",
                    ::gitty_hub::BASE_URL,
                    #path_str,
                ));

                #accept_header

                let request = if Self::METADATA.authentication && github.tkn.is_some() {
                    request.bearer_auth(github.tkn.as_ref().unwrap())
                } else {
                    request
                };

                #query

                #body

                let req = request.build()?;

                println!("URL {:?}", req.url().to_string());

                Ok(req)
            }
        }
    };

    let lifetime = if has_all_lifetimes {
        quote! { <'a> }
    } else {
        TokenStream::new()
    };
    let name = fields.iter().map(|f| &f.name);
    let ty = fields.iter().map(|f| &f.ty);
    let attr = req.req_fields.iter().map(|f| {
        let attr = f.attrs.iter().filter(|attr| !attr.path.is_ident("github"));
        quote! { #( #attr )* }
    });
    Ok(quote! {
        #[derive(Clone, Debug, ::serde::Serialize)]
        pub struct Request #lifetime { #( #attr pub #name: #ty ),* }

        #to_reqwest
    })
}
