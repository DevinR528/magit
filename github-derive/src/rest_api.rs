use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    visit::Visit,
    Attribute, Field, Ident, Lifetime, Lit, LitStr, Token, Type,
};

pub(crate) fn expand(api: GithubInput) -> syn::Result<TokenStream> {
    let request = expand_request(&api.request, &api.metadata)?;
    let response = expand_response(&api.response, &api.metadata)?;
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
        const METADATA: ::magit::cmds::MetaData = ::magit::cmds::MetaData {
            description: #description,
            method: ::reqwest::Method::#method,
            path: #path,
            name: #name,
            authentication: #authentication,
        };
    })
}

fn expand_request(req: &Request, meta: &Metadata) -> syn::Result<TokenStream> {
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
        .collect::<syn::Result<Vec<_>>>()?;

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
                rocket::http::RawStr::new(&self.#path_var).percent_decode_lossy().to_string()
            });
            fmt_string.replace_range(start_of_segment..end_of_segment, "{}");
        }
    }
    let path_str = if fmt_args.is_empty() {
        quote! { Self::METADATA.path.to_owned() }
    } else {
        quote! {
            format_args!(#fmt_string, #(#fmt_args),*)
        }
    };

    let metadata = expand_metadata(meta)?;

    let to_reqwest = quote! {
        impl<'a> ::magit::cmds::GithubRequest for Request<'a> {
            #metadata
            type Response = Response;
            fn to_request(self) -> Result<::reqwest::Request, ::magit::cmds::GithubFailure> {
                todo!()
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
        pub struct Request<'a> { #( #attr #name: #ty ),* }

        #to_reqwest
    })
}

enum AttrArg {
    Path,
    Header(Ident),
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

fn expand_response(res: &Response, meta: &Metadata) -> syn::Result<TokenStream> {
    let field = res.res_fields.iter();

    let from_reqwest = quote! {
        impl<'a> ::magit::cmds::GithubResponse for Response<'a> {
            fn from_response(resp: &'a reqwest::Response) -> Result<Response<'a>, ::magit::cmds::GithubFailure> {
                ::serde_json::from_slice(&resp.bytes()).map_err(|_| ::magit::cmds::GithubFailure::Fail)
            }
        }
    };

    Ok(quote! {
        #[derive(Clone, Debug, ::serde::Deserialize)]
        pub struct Response<'a> { #( #field ),* }

        #from_reqwest
    })
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

    custom_keyword!(header);

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
}

#[derive(Debug)]
struct Response {
    resp_kw: kw::response,
    res_fields: Punctuated<Field, Token![,]>,
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

        let req_kw = input.parse::<kw::request>()?;
        let _ = input.parse::<Token![:]>()?;
        let field_values;
        braced!(field_values in input);
        let req_fields =
            field_values.parse_terminated::<Field, Token![,]>(Field::parse_named)?;

        let resp_kw = input.parse::<kw::response>()?;
        let _ = input.parse::<Token![:]>()?;
        let field_values;
        braced!(field_values in input);
        let res_fields =
            field_values.parse_terminated::<Field, Token![,]>(Field::parse_named)?;
        Ok(Self {
            metadata: metadata(fields, meta_kw)?,
            request: Request { req_fields, req_kw },
            response: Response { res_fields, resp_kw },
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

fn parse<T: Parse>(
    input: &ParseStream<'_>,
    val: fn(T) -> FieldValue,
) -> syn::Result<FieldValue> {
    let _: Token![:] = input.parse()?;
    Ok(val(input.parse()?))
}
