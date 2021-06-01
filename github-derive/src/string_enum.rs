use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Fields, FieldsNamed, FieldsUnnamed, ItemEnum, LitStr, Variant};

mod case;
mod parse;

use case::RenameRule;
use parse::{RenameAllAttr, RenameAttr};

pub fn expand_enum_as_ref_str(input: &ItemEnum) -> syn::Result<TokenStream> {
    let enum_name = &input.ident;
    let rename_rule = get_rename_rule(input)?;
    let branches: Vec<_> = input
        .variants
        .iter()
        .map(|v| {
            let variant_name = &v.ident;
            let (field_capture, variant_str) = match (get_rename(v)?, &v.fields) {
                (None, Fields::Unit) => (
                    None,
                    rename_rule
                        .apply_to_variant(&variant_name.to_string())
                        .into_token_stream(),
                ),
                (Some(rename), Fields::Unit) => (None, rename.into_token_stream()),
                (None, Fields::Named(FieldsNamed { named: fields, .. }))
                | (None, Fields::Unnamed(FieldsUnnamed { unnamed: fields, .. })) => {
                    if fields.len() != 1 {
                        return Err(syn::Error::new_spanned(
                            v,
                            "multiple data fields are not supported",
                        ));
                    }

                    let capture = match &fields[0].ident {
                        Some(name) => quote! { { #name: inner } },
                        None => quote! { (inner) },
                    };

                    (Some(capture), quote! { inner })
                }
                (Some(_), _) => {
                    return Err(syn::Error::new_spanned(
                        v,
                        "github_enum(rename) is only allowed on unit variants",
                    ));
                }
            };

            Ok(quote! {
                #enum_name :: #variant_name #field_capture => #variant_str
            })
        })
        .collect::<syn::Result<_>>()?;

    Ok(quote! {
        #[automatically_derived]
        impl ::std::convert::AsRef<::std::primitive::str> for #enum_name {
            fn as_ref(&self) -> &::std::primitive::str {
                match self { #(#branches),* }
            }
        }
    })
}

pub fn expand_enum_from_string(input: &ItemEnum) -> syn::Result<TokenStream> {
    let enum_name = &input.ident;
    let rename_rule = get_rename_rule(input)?;
    let branches: Vec<_> = input
        .variants
        .iter()
        .map(|v| {
            let variant_name = &v.ident;
            let variant_str = match (get_rename(v)?, &v.fields) {
                (None, Fields::Unit) => Some(
                    rename_rule
                        .apply_to_variant(&variant_name.to_string())
                        .into_token_stream(),
                ),
                (Some(rename), Fields::Unit) => Some(rename.into_token_stream()),
                (None, Fields::Named(FieldsNamed { named: fields, .. }))
                | (None, Fields::Unnamed(FieldsUnnamed { unnamed: fields, .. })) => {
                    if fields.len() != 1 {
                        return Err(syn::Error::new_spanned(
                            v,
                            "multiple data fields are not supported",
                        ));
                    }
                    None
                }
                (Some(_), _) => {
                    return Err(syn::Error::new_spanned(
                        v,
                        "github_enum(rename) is only allowed on unit variants",
                    ));
                }
            };

            Ok(variant_str.map(|s| quote! { #s => #enum_name :: #variant_name }))
        })
        .collect::<syn::Result<_>>()?;

    // Remove `None` from the iterator to avoid emitting consecutive commas in repetition
    let branches = branches.iter().flatten();

    // TODO: this can error reflect that, I don't love the _Custom(String) variant but
    // maybe that is the best??
    Ok(quote! {
        impl<T> ::std::convert::From<T> for #enum_name
        where
            T: ::std::convert::AsRef<::std::primitive::str>
                + ::std::convert::Into<::std::string::String>
        {
            fn from(s: T) -> Self {
                match s.as_ref() {
                    #( #branches, )*
                    s => unreachable!("github changed a stringly typed enum {}", s),
                }
            }
        }
    })
}

pub fn expand_serialize_as_ref_str(ident: &Ident) -> syn::Result<TokenStream> {
    Ok(quote! {
        #[automatically_derived]
        impl ::serde::ser::Serialize for #ident {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                ::std::convert::AsRef::<::std::primitive::str>::as_ref(self).serialize(serializer)
            }
        }
    })
}

pub fn expand_deserialize_from_cow_str(ident: &Ident) -> syn::Result<TokenStream> {
    Ok(quote! {
        impl<'de> ::serde::de::Deserialize<'de> for #ident {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                type CowStr<'a> = ::std::borrow::Cow<'a, ::std::primitive::str>;

                let cow = ::ruma::serde::deserialize_cow_str(deserializer)?;
                Ok(::std::convert::From::<CowStr<'_>>::from(cow))
            }
        }
    })
}

pub fn expand_display_as_ref_str(ident: &Ident) -> syn::Result<TokenStream> {
    Ok(quote! {
        #[automatically_derived]
        impl ::std::fmt::Display for #ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(::std::convert::AsRef::<::std::primitive::str>::as_ref(self))
            }
        }
    })
}

pub fn get_rename_rule(input: &ItemEnum) -> syn::Result<RenameRule> {
    let rules: Vec<_> = input
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("github_enum"))
        .map(|attr| attr.parse_args::<RenameAllAttr>().map(RenameAllAttr::into_inner))
        .collect::<syn::Result<_>>()?;

    match rules.len() {
        0 => Ok(RenameRule::None),
        1 => Ok(rules[0]),
        _ => Err(syn::Error::new(
            Span::call_site(),
            "found multiple github_enum(rename_all) attributes",
        )),
    }
}

pub fn get_rename(input: &Variant) -> syn::Result<Option<LitStr>> {
    let renames: Vec<_> = input
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("github_enum"))
        .map(|attr| attr.parse_args::<RenameAttr>().map(RenameAttr::into_inner))
        .collect::<syn::Result<_>>()?;

    match renames.len() {
        0 | 1 => Ok(renames.into_iter().next()),
        _ => Err(syn::Error::new(
            Span::call_site(),
            "found multiple github_enum(rename) attributes",
        )),
    }
}
