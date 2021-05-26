use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, Field, Ident, Lit, LitStr, Token, Type, TypePath,
};

pub(crate) fn expand(api: GithubInput) -> syn::Result<TokenStream> {
    let request = expand_request(&api.request, &api.metadata)?;
    let response = expand_response(&api.response)?;
    Ok(quote! {
        #request
        #response
    })
}

fn expand_metadata(meta: &Metadata) -> syn::Result<TokenStream> {
    let Metadata { description, method, path, name, authentication, .. } = meta;

    let meta = format!("Metadata for the {} endpoint.", name.value());
    Ok(quote! {
        #[doc = #meta]
        ///
        #[doc = #description]
        const METADATA: ::magit::api::MetaData = ::magit::api::MetaData {
            description: #description,
            method: ::reqwest::Method::#method,
            path: #path,
            name: #name,
            authentication: #authentication,
        };
    })
}

fn expand_request(req: &Request, meta: &Metadata) -> syn::Result<TokenStream> {
    let metadata = expand_metadata(meta)?;

    let fields = req
        .req_fields
        .iter()
        .map(|f| {
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
                return Err(syn::Error::new_spanned(
                    f,
                    "only 1 github attribute allowed",
                ));
            }

            Ok(RequestField {
                attr: attrs.into_iter().next(),
                name: f.ident.clone().ok_or_else(|| {
                    syn::Error::new_spanned(f, "only 1 github attribute allowed")
                })?,
                ty: f.ty.clone(),
            })
        })
        .collect::<syn::Result<Vec<RequestField>>>()?;

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
                rocket::http::RawStr::new(&self.#path_var.to_string()).percent_decode_lossy().to_string()
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

    let mut query_calls =
        vec![quote! { let query_map = ::std::collections::HashMap::new(); }];
    for query in fields.iter().filter(|f| matches!(f.attr, Some(AttrArg::Query))) {
        let query_field: &Ident = &query.name;
        let name = Ident::new(
            query_field.to_string().trim_start_matches("r#"),
            query_field.span(),
        );

        let q_call = match &query.ty {
            Type::Path(TypePath { path, .. })
                if path.segments.first().map_or(false, |seg| seg.ident == "Option") =>
            {
                quote! {
                    if let Some(#name) = self.#query_field.as_ref() {
                        query_map.insert(stringify!(#name), #name.to_string());
                    }
                }
            }
            _ => quote! {
                query_map.insert(stringify!(#name), &self.#query_field.to_string());
            },
        };
        query_calls.push(q_call);
    }
    // If there are no query params don't emit anything to avoid putting a type on the
    // HashMap
    if query_calls.len() > 1 {
        query_calls.push(quote! { let request = request.query(&query_map); });
    } else {
        query_calls.clear();
    }

    let mut query_field = vec![];
    let mut init_field = vec![];
    let mut has_lifetime = false;
    for (field, orig) in fields
        .iter()
        .zip(req.req_fields.iter())
        .filter(|(f, _)| matches!(f.attr, Some(AttrArg::Query)))
    {
        if matches!(field.ty, Type::Reference(_)) {
            has_lifetime = true;
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
        let lifetime = if has_lifetime {
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
    let mut has_lifetime = false;
    for (field, orig) in fields
        .iter()
        .zip(req.req_fields.iter())
        .filter(|(f, _)| matches!(f.attr, Some(AttrArg::Body)))
    {
        if matches!(field.ty, Type::Reference(_)) {
            has_lifetime = true;
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
        let lifetime = if has_lifetime {
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
        impl<'a> ::magit::api::GithubRequest for Request<'a> {
            #metadata
            type Response = Response;
            fn to_request(
                self,
                github: &::magit::api::GithubClient
            ) -> Result<::reqwest::Request, ::magit::api::Error> {
                let request = github.request_builder(Self::METADATA.method, &::std::format!(
                    "{}{}",
                    ::magit::api::BASE_URL,
                    #path_str,
                ));

                let request = if let Some(accept) = self.accept.as_ref() {
                    request.header(::reqwest::header::ACCEPT, accept.to_string())
                } else {
                    request
                };

                let request = if Self::METADATA.authentication && github.tkn.is_some() {
                    request.bearer_auth(github.tkn.as_ref().unwrap())
                } else {
                    request
                };

                #query

                #body

                let req = request.build()?;
                println!("URL {:?}", req.url());
                // println!("URL {}", req.method());
                Ok(req)
            }
        }
    };

    let name = fields.iter().map(|f| &f.name);
    let ty = fields.iter().map(|f| &f.ty);
    let attr = req.req_fields.iter().map(|f| {
        let attr = f.attrs.iter().filter(|attr| !attr.path.is_ident("github"));
        quote! { #( #attr )* }
    });
    Ok(quote! {
        #[derive(Clone, Debug, ::serde::Serialize)]
        pub struct Request<'a> { #( #attr pub #name: #ty ),* }

        #to_reqwest
    })
}

enum AttrArg {
    Path,
    Header(Ident),
    Query,
    Body,
}

impl Parse for AttrArg {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::path) {
            let _ = input.parse::<kw::path>()?;
            Ok(AttrArg::Path)
        } else if lookahead.peek(kw::header) {
            let _ = input.parse::<kw::header>()?;
            let _ = input.parse::<Token![=]>()?;
            Ok(AttrArg::Header(input.parse()?))
        } else if lookahead.peek(kw::query) {
            let _ = input.parse::<kw::query>()?;
            Ok(AttrArg::Query)
        } else if lookahead.peek(kw::body) {
            let _ = input.parse::<kw::body>()?;
            Ok(AttrArg::Body)
        } else {
            Err(lookahead.error())
        }
    }
}

struct RequestField {
    attr: Option<AttrArg>,
    name: Ident,
    ty: Type,
}

fn expand_response(res: &Response) -> syn::Result<TokenStream> {
    let custom_attr = res.attrs.iter().find_map(|attr| {
        attr.path
            .is_ident("github")
            .then(|| attr.parse_args::<DeserAttr>().ok())
            .flatten()
    });
    let (impl_deser, derive_deser) =
        if let Some(DeserAttr::With(custom_deser)) = custom_attr {
            let s = custom_deser.value().replace("\"", "");
            let raw_path = s.split("::").flat_map(|seg| {
                if seg.is_empty() {
                    TokenStream::new()
                } else {
                    let seg = format_ident!("{}", seg);
                    quote! { ::#seg }
                }
            });
            let deser_fn = quote! { #( #raw_path )* (deser) };
            (
                quote! {
                    impl<'de> ::serde::de::Deserialize<'de> for Response {
                        fn deserialize<D>(deser: D) -> Result<Response, D::Error>
                        where
                            D: ::serde::de::Deserializer<'de>,
                        {
                            #deser_fn
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

    let from_reqwest = quote! {
        #[::rocket::async_trait]
        impl ::magit::api::GithubResponse for Response {
            async fn from_response(resp: reqwest::Response) -> Result<Response, ::magit::api::Error> {
                ::magit::api::from_status(resp.status())?;

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
        }
    };

    Ok(quote! {
        #[derive(Clone, Debug, #derive_deser)]
        #( #attr )*
        pub struct Response { #( #field ),* }

        #impl_deser

        #from_reqwest
    })
}

#[derive(Clone, Debug)]
enum DeserAttr {
    With(LitStr),
}

impl Parse for DeserAttr {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::with) {
            let _ = input.parse::<kw::with>()?;
            let _ = input.parse::<Token![=]>()?;
            Ok(Self::With(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

fn set_field<T: ToTokens>(field: &mut Option<T>, value: T) -> syn::Result<()> {
    match field {
        Some(existing_value) => {
            let mut error = syn::Error::new_spanned(value, "duplicate field assignment");
            error.combine(syn::Error::new_spanned(existing_value, "first one here"));
            Err(error)
        }
        None => {
            *field = Some(value);
            Ok(())
        }
    }
}

mod kw {
    use syn::custom_keyword;

    custom_keyword!(description);
    custom_keyword!(method);
    custom_keyword!(path);
    custom_keyword!(name);
    custom_keyword!(authentication);

    custom_keyword!(with);

    custom_keyword!(header);
    custom_keyword!(query);
    custom_keyword!(body);

    custom_keyword!(metadata);
    custom_keyword!(request);
    custom_keyword!(response);
}

#[derive(Debug)]
struct Metadata {
    meta_kw: kw::metadata,
    description: LitStr,
    method: Ident,
    path: LitStr,
    name: LitStr,
    authentication: Lit,
}

#[derive(Debug)]
struct Request {
    req_kw: kw::request,
    req_fields: Punctuated<Field, Token![,]>,
    attrs: Vec<Attribute>,
}

#[derive(Debug)]
struct Response {
    resp_kw: kw::response,
    res_fields: Punctuated<Field, Token![,]>,
    attrs: Vec<Attribute>,
}

#[derive(Debug)]
pub(crate) struct GithubInput {
    metadata: Metadata,
    request: Request,
    response: Response,
}

impl Parse for GithubInput {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let meta_kw = input.parse::<kw::metadata>()?;
        let _ = input.parse::<Token![:]>()?;
        let field_values;
        braced!(field_values in input);
        let fields =
            field_values.parse_terminated::<FieldValue, Token![,]>(FieldValue::parse)?;

        let req_attrs = input.call(Attribute::parse_outer)?;
        let req_kw = input.parse::<kw::request>()?;
        let _ = input.parse::<Token![:]>()?;
        let field_values;
        braced!(field_values in input);
        let req_fields =
            field_values.parse_terminated::<Field, Token![,]>(Field::parse_named)?;

        let res_attrs = input.call(Attribute::parse_outer)?;
        let resp_kw = input.parse::<kw::response>()?;
        let _ = input.parse::<Token![:]>()?;
        let field_values;
        braced!(field_values in input);
        let res_fields =
            field_values.parse_terminated::<Field, Token![,]>(Field::parse_named)?;
        Ok(Self {
            metadata: metadata(fields, meta_kw)?,
            request: Request { req_fields, req_kw, attrs: req_attrs },
            response: Response { res_fields, resp_kw, attrs: res_attrs },
        })
    }
}

fn metadata(
    fields: Punctuated<FieldValue, Token![,]>,
    meta_kw: kw::metadata,
) -> syn::Result<Metadata> {
    let mut doc = None;
    let mut method = None;
    let mut path = None;
    let mut name = None;
    let mut auth = None;

    for field in fields {
        match field.clone() {
            FieldValue::Description(desc) => set_field(&mut doc, desc)?,
            FieldValue::Method(meth) => set_field(&mut method, meth)?,
            FieldValue::Path(p) => set_field(&mut path, p)?,
            FieldValue::Name(n) => set_field(&mut name, n)?,
            FieldValue::Authentication(a) => set_field(&mut auth, a)?,
        };
    }
    let description = doc
        .ok_or_else(|| syn::Error::new_spanned(meta_kw, "missing `description` field"))?;
    let method = method
        .ok_or_else(|| syn::Error::new_spanned(meta_kw, "missing `method` field"))?;
    let path =
        path.ok_or_else(|| syn::Error::new_spanned(meta_kw, "missing `path` field"))?;
    let name =
        name.ok_or_else(|| syn::Error::new_spanned(meta_kw, "missing `name` field"))?;
    let authentication = auth.ok_or_else(|| {
        syn::Error::new_spanned(meta_kw, "missing `authentication` field")
    })?;

    Ok(Metadata { meta_kw, description, method, path, name, authentication })
}

#[derive(Clone, Debug)]
enum FieldValue {
    Description(LitStr),
    Method(Ident),
    Path(LitStr),
    Name(LitStr),
    Authentication(Lit),
}

impl Parse for FieldValue {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::description) {
            let _: kw::description = input.parse()?;
            parse(&input, Self::Description)
        } else if lookahead.peek(kw::method) {
            let _: kw::method = input.parse()?;
            parse(&input, Self::Method)
        } else if lookahead.peek(kw::path) {
            let _: kw::path = input.parse()?;
            parse(&input, Self::Path)
        } else if lookahead.peek(kw::name) {
            let _: kw::name = input.parse()?;
            parse(&input, Self::Name)
        } else if lookahead.peek(kw::authentication) {
            let _: kw::authentication = input.parse()?;
            parse(&input, Self::Authentication)
        } else {
            Err(lookahead.error())
        }
    }
}

fn parse<T: Parse, U>(input: &ParseStream<'_>, val: fn(T) -> U) -> syn::Result<U> {
    let _: Token![:] = input.parse()?;
    Ok(val(input.parse()?))
}
