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

use crate::attr::{FieldKind, Fields};

use proc_macro2::Ident;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Data, DataStruct};

static DERIVED_MIRROR_STRUCT_PREFIX: &str = "KeyableDerivedMirrorOf_";

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
    let impl_generics = add_generic_bounds(&input.generics, quote!(::keypath::TypedKeyable));
    let (_, ty_generics, where_clause) = &input.generics.split_for_impl();

    let fields = Fields::parse_ast(&s.fields)?;
    let get_field_arms = fields.iter().map(|fld| fld.match_arms(quote!(get_field)));
    let get_mut_field_arms = fields
        .iter()
        .map(|fld| fld.match_arms(quote!(get_field_mut)));

    let (fragment_decl, typed_trait_decl) = mirror_struct(ident, &input.generics, &fields)?;
    let res = quote! {
        impl<#impl_generics> ::keypath::RawKeyable for #ident #ty_generics #where_clause {
            fn as_any(&self) -> &dyn ::std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn ::std::any::Any {
                self
            }

            fn get_field(&self, ident: &[::keypath::internals::PathComponent]) -> Result<&dyn ::keypath::RawKeyable, ::keypath::FieldError> {
                match ident.split_first() {
                None => Ok(self),
                 #( #get_field_arms )*
                    Some((field, rest)) => Err(
                        ::keypath::FieldErrorKind::InvalidField(field.clone()).into_error(self, rest.len())
                    ),

                }
            }

            fn get_field_mut(&mut self, ident: &[::keypath::internals::PathComponent]) -> Result<&mut dyn ::keypath::RawKeyable, ::keypath::FieldError> {
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

        #fragment_decl

        impl<#impl_generics> ::keypath::TypedKeyable for #ident #ty_generics #where_clause {
            #typed_trait_decl
        }
    };
    Ok(res)
}

fn mirror_struct(
    base_ident: &Ident,
    generics: &syn::Generics,
    fields: &Fields,
) -> Result<(proc_macro2::TokenStream, proc_macro2::TokenStream), syn::Error> {
    let (_, ty_generics, _) = generics.split_for_impl();
    let impl_generics = add_generic_bounds(generics, quote!(::keypath::TypedKeyable));
    let mirror_ident = mirror_ident_for_base_ident(base_ident);

    let field_decls = fields.generate_mirror_decls();
    let struct_decl = match fields.kind {
        FieldKind::Named => {
            quote!(pub struct #mirror_ident #ty_generics{#field_decls})
        }

        FieldKind::Unnamed => {
            quote!(pub struct #mirror_ident #ty_generics(#field_decls);)
        }
    };
    let struct_decl = quote!(#[allow(non_camel_case_types)]
        #struct_decl);

    let fragments = fields
        .iter()
        .map(|fld| fld.validation_fn_ident())
        .collect::<Vec<_>>();
    let field_types = fields.iter().map(|fld| &fld.ty).collect::<Vec<_>>();
    let methods = quote!(
        #(pub fn #fragments(self) -> <#field_types as ::keypath::TypedKeyable>::PathFragment {
            <#field_types as ::keypath::TypedKeyable>::get()
        })*

        pub fn to_key_path_with_root<Root>(self, fields: &'static [::keypath::internals::PathComponent]) -> ::keypath::KeyPath<Root, #base_ident #ty_generics> {
            ::keypath::KeyPath::__conjure_from_abyss(fields)
        }
    );

    let struct_field_init = fields.generate_mirror_inits();
    let struct_init = match fields.kind {
        FieldKind::Named => quote!(Self {#struct_field_init}),
        FieldKind::Unnamed => quote!(Self (#struct_field_init)),
    };

    let tokens = quote!(
        #struct_decl

        impl< #impl_generics> #mirror_ident #ty_generics {
            fn new() -> Self {
                #struct_init
            }

            #methods
        }
    );

    let trait_impl = quote!(
        type PathFragment = #mirror_ident #ty_generics;
        fn get() -> #mirror_ident #ty_generics{
            #mirror_ident::new()
        }
    );
    Ok((tokens, trait_impl))
}

fn mirror_ident_for_base_ident(ident: &Ident) -> Ident {
    Ident::new(
        &format!("{}{}", DERIVED_MIRROR_STRUCT_PREFIX, ident),
        ident.span(),
    )
}

fn add_generic_bounds(
    generics: &syn::Generics,
    with_bounds: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let res = generics.params.iter().map(|gp| {
        use syn::GenericParam::*;
        match gp {
            Type(ty) => {
                let ident = &ty.ident;
                let bounds = &ty.bounds;
                if bounds.is_empty() {
                    quote_spanned!(ty.span()=> #ident : #with_bounds)
                } else {
                    quote_spanned!(ty.span()=> #ident : #bounds + #with_bounds)
                }
            }
            Lifetime(lf) => quote!(#lf),
            Const(cst) => quote!(#cst),
        }
    });

    quote!( #( #res, )* )
}
