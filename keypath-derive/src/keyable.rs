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

use crate::attr::Fields;

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
    let impl_generics = generics_bounds(&input.generics);
    let (_, ty_generics, where_clause) = &input.generics.split_for_impl();

    let fields = Fields::parse_ast(&s.fields)?;
    let get_field_arms = fields.iter().map(|fld| fld.match_arms(quote!(get_field)));
    let get_mut_field_arms = fields
        .iter()
        .map(|fld| fld.match_arms(quote!(get_field_mut)));

    let (fragment_decl, typed_trait_decl) = path_fragment_struct(ident, &input.generics, &fields)?;
    let res = quote! {
        impl<#impl_generics> ::keypath::RawKeyable for #ident #ty_generics #where_clause {
            fn as_any(&self) -> &dyn ::std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn ::std::any::Any {
                self
            }

            fn get_field(&self, ident: &[::keypath::PathComponent]) -> Result<&dyn ::keypath::RawKeyable, ::keypath::FieldError> {
                match ident.split_first() {
                None => Ok(self),
                 #( #get_field_arms )*
                    Some((field, rest)) => Err(
                        ::keypath::FieldErrorKind::InvalidField(field.clone()).into_error(self, rest.len())
                    ),

                }
            }

            fn get_field_mut(&mut self, ident: &[::keypath::PathComponent]) -> Result<&mut dyn ::keypath::RawKeyable, ::keypath::FieldError> {
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
    //eprintln!("TOKENS: {}", res);
    Ok(res)
}

fn path_fragment_struct(
    base_ident: &Ident,
    generics: &syn::Generics,
    fields: &Fields,
) -> Result<(proc_macro2::TokenStream, proc_macro2::TokenStream), syn::Error> {
    let (_, ty_generics, _) = generics.split_for_impl();
    let fragment_type = fragment_ident_for_base_ident(base_ident);
    let fragments = fields
        .iter()
        .map(|fld| fld.validation_fn_ident())
        .collect::<Vec<_>>();
    let field_types = fields.iter().map(|fld| &fld.ty).collect::<Vec<_>>();
    let tokens = quote!(
        #(pub fn #fragments(self) -> <#field_types as ::keypath::TypedKeyable>::PathFragment {
            <#field_types as ::keypath::TypedKeyable>::fragment()
        })*

        pub fn to_key_path_with_root<Root>(self, fields: &'static [::keypath::PathComponent]) -> ::keypath::KeyPath<Root, #base_ident #ty_generics> {
            ::keypath::KeyPath::__conjure_from_abyss(fields)
        }
    );
    fragment_decl_header(&fragment_type, generics, tokens)
}

fn fragment_decl_header(
    ident: &Ident,
    generics: &syn::Generics,
    methods: proc_macro2::TokenStream,
) -> Result<(proc_macro2::TokenStream, proc_macro2::TokenStream), syn::Error> {
    let mut type_params = Vec::new();
    let mut impl_params = Vec::new();
    let mut phantom_decls = Vec::new();
    let mut phantom_inits = Vec::new();
    for param in generics.params.iter() {
        match param {
            syn::GenericParam::Type(syn::TypeParam { ident, .. }) => {
                type_params.push(quote!(#ident));
                impl_params.push(quote!(#ident: ::keypath::TypedKeyable));
                phantom_decls.push(quote!(::std::marker::PhantomData<#ident>));
                phantom_inits.push(quote!(::std::marker::PhantomData));
            }
            syn::GenericParam::Const(param) => {
                return Err(syn::Error::new(
                    param.span(),
                    "Keypaths don't currently support const generics",
                ))
            }
            syn::GenericParam::Lifetime(param) => {
                return Err(syn::Error::new(
                    param.span(),
                    "Keypaths don't currently support lifetime paramaters",
                ))
            }
        }
    }

    let tokens = quote!(
        #[allow(non_camel_case_types)]
        pub struct #ident<#( #type_params ),*>(#( #phantom_inits ),*);
        impl<#( #type_params ),*> #ident<#( #type_params ),*> {
            fn new() -> Self {
                #ident(#( #phantom_inits ),*)
            }

            #methods
        }
    );

    let trait_impl = quote!(
        type PathFragment = #ident<#( #type_params ),*>;
        fn fragment() -> #ident<#( #type_params ),*> {
            #ident::new()
        }
    );
    Ok((tokens, trait_impl))
}

fn fragment_ident_for_base_ident(ident: &Ident) -> Ident {
    Ident::new(
        &format!("{}{}", DERIVED_MIRROR_STRUCT_PREFIX, ident),
        ident.span(),
    )
}

fn generics_bounds(generics: &syn::Generics) -> proc_macro2::TokenStream {
    let res = generics.params.iter().map(|gp| {
        use syn::GenericParam::*;
        match gp {
            Type(ty) => {
                let ident = &ty.ident;
                let bounds = &ty.bounds;
                if bounds.is_empty() {
                    quote_spanned!(ty.span()=> #ident : ::keypath::Keyable)
                } else {
                    quote_spanned!(ty.span()=> #ident : #bounds + ::keypath::Keyable)
                }
            }
            Lifetime(lf) => quote!(#lf),
            Const(cst) => quote!(#cst),
        }
    });

    quote!( #( #res, )* )
}
