// Copyright 2019 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! The implementation for #[derive(Data)]

use crate::attr::{Field, Fields, RawKeyableAttrs};

use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Data, DataStruct};

pub(crate) fn derive_keyable_impl(
    input: syn::DeriveInput,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    match &input.data {
        Data::Struct(s) => derive_struct(&input, s),
        Data::Enum(e) => Err(syn::Error::new(
            e.enum_token.span(),
            "Keyable cannot currently be derived for enums",
        )),
        Data::Union(u) => Err(syn::Error::new(
            u.union_token.span(),
            "Data implementations cannot be derived from unions",
        )),
    }
}

fn derive_struct(
    input: &syn::DeriveInput,
    s: &DataStruct,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let ident = &input.ident;
    let impl_generics = generics_bounds(&input.generics);
    let (_, ty_generics, where_clause) = &input.generics.split_for_impl();

    let fields = Fields::<RawKeyableAttrs>::parse_ast(&s.fields)?;

    let get_field_arms = fields.iter().map(|fld| fld.match_arms(quote!(get_field)));
    let get_mut_field_arms = fields
        .iter()
        .map(|fld| fld.match_arms(quote!(get_field_mut)));
    let res = quote! {
        impl<#impl_generics> ::keypath::RawKeyable for #ident #ty_generics #where_clause {
            fn as_any(&self) -> &dyn ::std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn ::std::any::Any {
                self
            }

            fn get_field(&self, ident: &[::keypath::Field]) -> Result<&dyn ::keypath::RawKeyable, ::keypath::FieldError> {
                match ident.split_first() {
                None => Ok(self),
                 #( #get_field_arms )*
                    Some((field, rest)) => Err(
                        ::keypath::FieldErrorKind::InvalidField(field.clone()).into_error(self, rest.len())
                    ),

                }
            }

            fn get_field_mut(&mut self, ident: &[::keypath::Field]) -> Result<&mut dyn ::keypath::RawKeyable, ::keypath::FieldError> {
                match ident.split_first() {
                None => Ok(self),
                #( #get_mut_field_arms )*
                    Some((field, rest)) => Err(
                        ::keypath::FieldErrorKind::InvalidField(field.clone()).into_error(self, rest.len())
                    ),

            }
        }
        }

        impl<#impl_generics> ::keypath::Keyable for #ident #ty_generics #where_clause {}
    };
    //eprintln!("TOKENS: {}", res);
    Ok(res)
}

fn generics_bounds(generics: &syn::Generics) -> proc_macro2::TokenStream {
    let res = generics.params.iter().map(|gp| {
        use syn::GenericParam::*;
        match gp {
            Type(ty) => {
                let ident = &ty.ident;
                let bounds = &ty.bounds;
                if bounds.is_empty() {
                    quote_spanned!(ty.span()=> #ident : ::druid::Data)
                } else {
                    quote_spanned!(ty.span()=> #ident : #bounds + ::druid::Data)
                }
            }
            Lifetime(lf) => quote!(#lf),
            Const(cst) => quote!(#cst),
        }
    });

    quote!( #( #res, )* )
}
