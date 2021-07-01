use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse_quote, punctuated::Punctuated, AngleBracketedGenericArguments, Data,
    DeriveInput, Field, Fields, GenericArgument, GenericParam, Generics, Meta, MetaList,
    NestedMeta, ParenthesizedGenericArguments, PathArguments, Type, TypeGenerics,
    TypePath, TypeReference, TypeSlice, Variant,
};

enum StructKind {
    Struct,
    Tuple,
}

enum DataKind {
    Struct(Vec<Field>, StructKind),
    Enum(Vec<Variant>),
    Unit,
}

pub(crate) fn expand(input: DeriveInput) -> syn::Result<TokenStream> {
    let mut emit_deserialize = true;
    let input_attrs = input
        .attrs
        .iter()
        .filter(|attr| {
            if attr.path.is_ident("no_deserialize") {
                emit_deserialize = false;
                false
            } else {
                true
            }
        })
        .collect::<Vec<_>>();

    let derives = if emit_deserialize {
        quote! {
            #[derive(Clone, Debug, ::serde::Deserialize)]
        }
    } else {
        quote! {
            #[derive(Clone, Debug)]
        }
    };
    let data = match input.data.clone() {
        Data::Union(_) => panic!("#[derive(Incoming)] does not support Union types"),
        Data::Enum(e) => DataKind::Enum(e.variants.into_iter().collect()),
        Data::Struct(s) => match s.fields {
            Fields::Named(fs) => DataKind::Struct(
                fs.named.into_iter().map(strip_lifetime_attrs).collect(),
                StructKind::Struct,
            ),
            Fields::Unnamed(fs) => DataKind::Struct(
                fs.unnamed.into_iter().map(strip_lifetime_attrs).collect(),
                StructKind::Tuple,
            ),
            Fields::Unit => DataKind::Unit,
        },
    };
    match data {
        DataKind::Unit => Ok(TokenStream::new()),
        DataKind::Enum(mut vars) => {
            let mut found_lifetime = false;
            for var in &mut vars {
                for field in &mut var.fields {
                    if strip_lifetimes(&mut field.ty)? {
                        found_lifetime = true;
                    }
                }
            }
            if !found_lifetime {
                return Ok(TokenStream::new());
            }

            let original_ident = &input.ident;
            let vis = input.vis;
            let doc = format!(
                "'Incoming' variant of [{ty}](enum.{ty}.html).",
                ty = &input.ident
            );
            let incoming_ident =
                format_ident!("Incoming{}", original_ident, span = Span::call_site());

            let mut gen_copy = input.generics.clone();
            let ty_gen = split_for_impl_remove_lifetimes(&mut gen_copy);

            Ok(quote! {
                #[doc = #doc]
                #derives
                #( #input_attrs )*
                #vis enum #incoming_ident #ty_gen { #( #vars, )* }
            })
        }
        DataKind::Struct(mut fields, struct_kind) => {
            let mut found_lifetime = false;
            for field in &mut fields {
                if !matches!(field.vis, syn::Visibility::Public(_)) {
                    return Err(syn::Error::new_spanned(
                        field,
                        "All fields must be marked `pub`",
                    ));
                }
                if strip_lifetimes(&mut field.ty)? {
                    found_lifetime = true;
                }
            }
            if !found_lifetime {
                return Ok(TokenStream::new());
            }

            let original_ident = &input.ident;
            let vis = input.vis;
            let doc = format!(
                "'Incoming' variant of [{ty}](struct.{ty}.html).",
                ty = &input.ident
            );
            let incoming_ident =
                format_ident!("Incoming{}", original_ident, span = Span::call_site());

            let mut gen_copy = input.generics.clone();
            let ty_gen = split_for_impl_remove_lifetimes(&mut gen_copy);

            let struct_def = match struct_kind {
                StructKind::Struct => quote! { { #(#fields,)* } },
                StructKind::Tuple => quote! { ( #(#fields,)* ); },
            };

            Ok(quote! {
                #[doc = #doc]
                #derives
                #( #input_attrs )*
                #vis struct #incoming_ident #ty_gen #struct_def
            })
        }
    }
}

fn strip_lifetime_attrs(mut f: Field) -> Field {
    for attr in &mut f.attrs {
        if attr.path.is_ident("serde") {
            if let Ok(Meta::List(MetaList { nested, .. })) = attr.parse_meta() {
                let filtered = nested
                    .into_iter()
                    .filter(|meta| {
                        !matches!(
                            meta,
                            NestedMeta::Meta(Meta::Path(p)) if p.is_ident("borrow")
                        )
                    })
                    .collect::<Vec<NestedMeta>>();
                *attr = parse_quote! { #[serde(#( #filtered, )*)]}
            }
        }
    }
    f
}

fn split_for_impl_remove_lifetimes(generics: &mut Generics) -> TypeGenerics<'_> {
    generics.params = generics
        .params
        .clone()
        .into_iter()
        .filter(|param| !matches!(param, GenericParam::Lifetime(_)))
        .collect();

    let (_, ty_gen, _) = generics.split_for_impl();
    ty_gen
}

fn strip_lifetimes(field_type: &mut Type) -> syn::Result<bool> {
    Ok(match field_type {
        // T<'a> -> IncomingT
        // The IncomingT has to be declared by the user of this derive macro.
        Type::Path(TypePath { path, .. }) => {
            let mut has_lifetimes = false;
            let mut is_lifetime_generic = false;

            for seg in &mut path.segments {
                // strip generic lifetimes
                match &mut seg.arguments {
                    PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        args,
                        ..
                    }) => {
                        *args = args
                            .clone()
                            .into_iter()
                            .map(|mut ty| {
                                if let GenericArgument::Type(ty) = &mut ty {
                                    if strip_lifetimes(ty)? {
                                        has_lifetimes = true;
                                    };
                                }
                                Ok(ty)
                            })
                            .filter(|arg| {
                                if let Ok(GenericArgument::Lifetime(_)) = arg {
                                    is_lifetime_generic = true;
                                    false
                                } else {
                                    true
                                }
                            })
                            .collect::<syn::Result<Punctuated<_, _>>>()?;
                    }
                    PathArguments::Parenthesized(ParenthesizedGenericArguments {
                        inputs,
                        ..
                    }) => {
                        *inputs = inputs
                            .clone()
                            .into_iter()
                            .map(|mut ty| {
                                if strip_lifetimes(&mut ty)? {
                                    has_lifetimes = true;
                                };
                                Ok(ty)
                            })
                            .collect::<syn::Result<Punctuated<Type, _>>>()?;
                    }
                    _ => {}
                }
            }

            // If a type has a generic lifetime parameter there must be an `Incoming`
            // variant of that type.
            if is_lifetime_generic {
                if let Some(name) = path.segments.last_mut() {
                    if name.ident == "Cow" {
                        if let PathArguments::AngleBracketed(
                            AngleBracketedGenericArguments { args, .. },
                        ) = &name.arguments
                        {
                            let must_be_str = args.iter().any(|arg| {
                                if let GenericArgument::Type(Type::Path(path)) = arg {
                                    path.path.is_ident("str")
                                } else {
                                    false
                                }
                            });
                            if must_be_str {
                                name.ident = Ident::new("String", name.ident.span());
                                name.arguments = PathArguments::None;
                            } else {
                                return Err(syn::Error::new_spanned(field_type, ""));
                            }
                        }
                    } else {
                        let incoming_ty_ident = format_ident!("Incoming{}", name.ident);
                        name.ident = incoming_ty_ident;
                    }
                }
            }

            has_lifetimes || is_lifetime_generic
        }
        Type::Reference(TypeReference { elem, .. }) => {
            let special_replacement = match &mut **elem {
                Type::Path(ty) => {
                    let path = &ty.path;
                    let last_seg = path.segments.last().unwrap();

                    if last_seg.ident == "str" {
                        // &str -> String
                        Some(parse_quote! { ::std::string::String })
                    } else if last_seg.ident == "Path" {
                        Some(parse_quote! { ::std::path::PathBuf })
                    } else if last_seg.ident == "ApiLink"
                    // TODO: any more id types
                        || last_seg.ident == "ServerName"
                    {
                        // The identifiers that need to be boxed `Box<T>` since they
                        // are DST's.
                        Some(parse_quote! { ::std::boxed::Box<#path> })
                    } else {
                        None
                    }
                }
                // &[T] -> Vec<T>
                Type::Slice(TypeSlice { elem, .. }) => {
                    // Recursively strip the lifetimes of the slice's elements.
                    strip_lifetimes(&mut *elem)?;
                    Some(parse_quote! { Vec<#elem> })
                }
                _ => None,
            };

            *field_type = match special_replacement {
                Some(ty) => ty,
                None => {
                    // Strip lifetimes of `elem`.
                    strip_lifetimes(elem)?;
                    // Replace reference with `elem`.
                    (**elem).clone()
                }
            };

            true
        }
        Type::Tuple(syn::TypeTuple { elems, .. }) => {
            let mut has_lifetime = false;
            for elem in elems {
                if strip_lifetimes(elem)? {
                    has_lifetime = true;
                }
            }
            has_lifetime
        }
        _ => false,
    })
}
