use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    AngleBracketedGenericArguments, Attribute, Field, GenericArgument, Ident, Lit,
    LitStr, ParenthesizedGenericArguments, Path, PathArguments, Token, Type, TypePath,
    TypeReference,
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
        #[::rocket::async_trait]
        impl ::magit::api::GithubResponse for Response {
            async fn from_response(resp: reqwest::Response) -> Result<Response, ::magit::api::Error> {
                ::magit::api::from_status(resp.status())?;

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

#[derive(Clone, Debug)]
enum DeserAttr {
    With(Path),
    ForwardToBody(Ident),
    ForwardToBodyWith(Ident, Path),
}

impl Parse for DeserAttr {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::with) {
            let _ = input.parse::<kw::with>()?;
            let _ = input.parse::<Token![=]>()?;
            Ok(Self::With(input.parse()?))
        } else if lookahead.peek(kw::forward_to_body) {
            let _ = input.parse::<kw::forward_to_body>()?;
            let _ = input.parse::<Token![=]>()?;
            Ok(Self::ForwardToBody(input.parse()?))
        } else if lookahead.peek(kw::forward_to_body_with) {
            let _ = input.parse::<kw::forward_to_body_with>()?;
            let _ = input.parse::<Token![=]>()?;
            let tuple;
            syn::parenthesized!(tuple in input);
            let field = tuple.parse()?;
            let _ = tuple.parse::<Token![,]>()?;
            let call = tuple.parse()?;
            Ok(Self::ForwardToBodyWith(field, call))
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
    custom_keyword!(forward_to_body);
    custom_keyword!(forward_to_body_with);

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

fn check_for_lifetimes(field_type: &Type) -> syn::Result<bool> {
    Ok(match field_type {
        // T<'a> -> IncomingT
        // The IncomingT has to be declared by the user of this derive macro.
        Type::Path(TypePath { path, .. }) => {
            let mut has_lifetimes = false;
            let mut is_lifetime_generic = false;

            for seg in &path.segments {
                // strip generic lifetimes
                match &seg.arguments {
                    PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        args,
                        ..
                    }) => {
                        for ty in args.iter() {
                            if let GenericArgument::Type(ty) = &ty {
                                if check_for_lifetimes(ty)? {
                                    has_lifetimes = true;
                                };
                            }
                            if let GenericArgument::Lifetime(_) = ty {
                                is_lifetime_generic = true;
                            }
                        }
                    }
                    PathArguments::Parenthesized(ParenthesizedGenericArguments {
                        inputs,
                        ..
                    }) => {
                        for ty in inputs.iter() {
                            if check_for_lifetimes(ty)? {
                                has_lifetimes = true;
                            }
                        }
                    }
                    _ => {}
                }
            }

            has_lifetimes || is_lifetime_generic
        }
        Type::Reference(TypeReference { .. }) => true,
        Type::Tuple(syn::TypeTuple { elems, .. }) => {
            let mut has_lifetime = false;
            for elem in elems {
                if check_for_lifetimes(elem)? {
                    has_lifetime = true;
                }
            }
            has_lifetime
        }
        _ => false,
    })
}
