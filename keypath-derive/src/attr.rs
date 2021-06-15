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

//! parsing helpers

use proc_macro2::{Ident, Literal, Span, TokenStream, TokenTree};
use quote::quote;
use syn::Error;

/// The fields for a struct or an enum variant.
pub struct Fields<Attrs> {
    pub kind: FieldKind,
    fields: Vec<Field<Attrs>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldKind {
    Named,
    // this also covers Unit; we determine 'unit-ness' based on the number
    // of fields.
    Unnamed,
}

#[derive(Debug)]
pub enum FieldIdent {
    Named(String),
    Unnamed(usize),
}

pub struct Field<Attrs> {
    pub ident: FieldIdent,
    pub ty: syn::Type,

    pub attrs: Attrs,
}

#[derive(Debug)]
pub struct RawKeyableAttrs;

impl Fields<RawKeyableAttrs> {
    pub fn parse_ast(fields: &syn::Fields) -> Result<Self, Error> {
        let kind = match fields {
            syn::Fields::Named(_) => FieldKind::Named,
            syn::Fields::Unnamed(_) | syn::Fields::Unit => FieldKind::Unnamed,
        };

        let fields = fields
            .iter()
            .enumerate()
            .map(|(i, field)| Field::<RawKeyableAttrs>::parse_ast(field, i))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Fields { kind, fields })
    }
}

impl<Attrs> Fields<Attrs> {
    pub fn iter(&self) -> impl Iterator<Item = &Field<Attrs>> {
        self.fields.iter()
    }
}

impl Field<RawKeyableAttrs> {
    pub fn parse_ast(field: &syn::Field, index: usize) -> Result<Self, Error> {
        let ident = match field.ident.as_ref() {
            Some(ident) => FieldIdent::Named(ident.to_string().trim_start_matches("r#").to_owned()),
            None => FieldIdent::Unnamed(index),
        };

        let ty = field.ty.clone();

        Ok(Field {
            ident,
            ty,
            attrs: RawKeyableAttrs,
        })
    }
}

impl<Attrs> Field<Attrs> {
    pub fn ident_tokens(&self) -> TokenTree {
        match self.ident {
            FieldIdent::Named(ref s) => Ident::new(&s, Span::call_site()).into(),
            FieldIdent::Unnamed(num) => Literal::usize_unsuffixed(num).into(),
        }
    }

    pub fn match_variant(&self) -> TokenStream {
        match &self.ident {
            FieldIdent::Named(s) => quote!(::keypath::Field::Named(#s)),
            FieldIdent::Unnamed(idx) => quote!(::keypath::Field::Ord(#idx)),
        }
    }
}
