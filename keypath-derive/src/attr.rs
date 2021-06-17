//! parsing helpers

use proc_macro2::{Ident, Literal, Span, TokenStream, TokenTree};
use quote::quote;
use syn::Error;

use super::shared::FieldIdent;

/// The fields for a struct or an enum variant.
pub struct Fields {
    pub kind: FieldKind,
    fields: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldKind {
    Named,
    // this also covers Unit; we determine 'unit-ness' based on the number
    // of fields.
    Unnamed,
}

pub struct Field {
    pub ident: FieldIdent,
    pub ty: syn::Type,
    span: Span,
    //pub attrs: Attrs,
}

impl Fields {
    pub fn parse_ast(fields: &syn::Fields) -> Result<Self, Error> {
        let kind = match fields {
            syn::Fields::Named(_) => FieldKind::Named,
            syn::Fields::Unnamed(_) | syn::Fields::Unit => FieldKind::Unnamed,
        };

        let fields = fields
            .iter()
            .enumerate()
            .map(|(i, field)| Field::parse_ast(field, i))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Fields { kind, fields })
    }
}

impl Fields {
    pub fn iter(&self) -> impl Iterator<Item = &Field> {
        self.fields.iter()
    }
}

impl Field {
    pub fn parse_ast(field: &syn::Field, index: usize) -> Result<Self, Error> {
        let ident = match field.ident.as_ref() {
            Some(ident) => FieldIdent::Named(ident.to_string().trim_start_matches("r#").to_owned()),
            None => FieldIdent::Unnamed(index),
        };

        let ty = field.ty.clone();
        let span = field
            .ident
            .as_ref()
            .map(|id| id.span())
            .unwrap_or_else(Span::call_site);

        Ok(Field { ident, ty, span })
    }
}

impl Field {
    fn field_tokens(&self) -> TokenTree {
        match self.ident {
            FieldIdent::Named(ref s) => Ident::new(&s, Span::call_site()).into(),
            FieldIdent::Unnamed(num) => Literal::usize_unsuffixed(num).into(),
        }
    }

    pub fn validation_fn_ident(&self) -> proc_macro2::Ident {
        let name = self.ident.validation_fn_name();
        Ident::new(&name, self.span)
    }

    pub fn match_arms(&self, method_tokens: TokenStream) -> TokenStream {
        let field = self.field_tokens();
        let variant = self.ident.path_component_tokens();
        quote!(Some((#variant, rest)) => self.#field.#method_tokens(rest),)
    }
}
