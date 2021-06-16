//! parsing helpers

use proc_macro2::{Ident, Literal, Span, TokenStream, TokenTree};
use quote::quote;
use syn::Error;

//const FRAGMENT_PREFIX: &str = "__keypath_derived_";
//const VALIDATE_PREFIX: &str = "__keypath_validate_";

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

impl FieldIdent {
    pub fn validation_fn_name(&self) -> TokenStream {
        let validation_name = match self {
            FieldIdent::Named(s) => s.clone(),
            FieldIdent::Unnamed(idx) => idx.to_string(),
        };
        //let ident = format!("{}{}", VALIDATE_PREFIX, validation_name);
        TokenTree::Ident(Ident::new(&validation_name, Span::call_site())).into()
    }

    pub fn to_field_tokens(&self) -> TokenStream {
        match self {
            FieldIdent::Named(s) => quote!(::keypath::Field::Name(#s)),
            FieldIdent::Unnamed(idx) => quote!(::keypath::Field::Ord(#idx)),
        }
    }
}

impl<Attrs> Field<Attrs> {
    pub fn ident_tokens(&self) -> TokenTree {
        match self.ident {
            FieldIdent::Named(ref s) => Ident::new(&s, Span::call_site()).into(),
            FieldIdent::Unnamed(num) => Literal::usize_unsuffixed(num).into(),
        }
    }

    pub fn match_arms(&self, method_tokens: TokenStream) -> TokenStream {
        let field = self.ident_tokens();
        let variant = self.field_variant();
        quote!(Some((#variant, rest)) => self.#field.#method_tokens(rest),)
    }

    pub fn field_variant(&self) -> TokenStream {
        match &self.ident {
            FieldIdent::Named(s) => quote!(::keypath::Field::Name(#s)),
            FieldIdent::Unnamed(idx) => quote!(::keypath::Field::Ord(#idx)),
        }
    }
}
