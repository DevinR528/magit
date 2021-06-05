use quote::ToTokens;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    AngleBracketedGenericArguments, Attribute, Field, GenericArgument, Ident, Lit,
    LitStr, ParenthesizedGenericArguments, Path, PathArguments, Token, Type, TypePath,
    TypeReference,
};

pub(crate) mod kw {
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
pub(crate) struct GithubInput {
    pub(crate) metadata: Metadata,
    pub(crate) request: Request,
    pub(crate) response: Response,
}

#[derive(Debug)]
pub(crate) struct Metadata {
    pub(crate) meta_kw: kw::metadata,
    pub(crate) description: LitStr,
    pub(crate) method: Ident,
    pub(crate) path: LitStr,
    pub(crate) name: LitStr,
    pub(crate) authentication: Lit,
}

#[derive(Debug)]
pub(crate) struct Request {
    pub(crate) req_kw: kw::request,
    pub(crate) req_fields: Punctuated<Field, Token![,]>,
    pub(crate) attrs: Vec<Attribute>,
}

#[derive(Debug)]
pub(crate) struct Response {
    pub(crate) resp_kw: kw::response,
    pub(crate) res_fields: Punctuated<Field, Token![,]>,
    pub(crate) attrs: Vec<Attribute>,
}

#[derive(Clone, Debug)]
pub(crate) enum FieldValue {
    Description(LitStr),
    Method(Ident),
    Path(LitStr),
    Name(LitStr),
    Authentication(Lit),
}

pub(crate) enum AttrArg {
    Path,
    Header(Ident),
    Query,
    Body,
}

#[derive(Clone, Debug)]
pub(crate) enum DeserAttr {
    With(Path),
    ForwardToBody(Ident),
    ForwardToBodyWith(Ident, Path),
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

pub(crate) fn check_for_lifetimes(field_type: &Type) -> syn::Result<bool> {
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
