use proc_macro2::{Ident, Literal, Span};
use quote::{quote, quote_spanned};

pub enum PathComponent {
    Field(FieldIdent),
    IndexInt(usize),
    IndexStr(String),
}

pub enum FieldIdent {
    Named(String),
    Unnamed(usize),
}

impl PathComponent {
    pub fn unnamed(idx: usize) -> Self {
        PathComponent::Field(FieldIdent::Unnamed(idx))
    }

    pub fn named(name: impl Into<String>) -> Self {
        PathComponent::Field(FieldIdent::Named(name.into()))
    }

    pub fn path_component_tokens(&self) -> proc_macro2::TokenStream {
        match self {
            PathComponent::Field(ident) => ident.path_component_tokens(),
            PathComponent::IndexInt(idx) => {
                quote!(::keypath::internals::PathComponent::IndexInt(#idx))
            }
            PathComponent::IndexStr(s) => quote!(::keypath::internals::PathComponent::IndexStr(#s)),
        }
    }

    pub fn to_tokens(&self, span: proc_macro2::Span) -> proc_macro2::TokenStream {
        match self {
            PathComponent::Field(FieldIdent::Named(ident)) => {
                let ident = Ident::new(&ident, Span::call_site());
                quote_spanned!(span=> .#ident.mirror())
            }
            PathComponent::Field(FieldIdent::Unnamed(ident)) => {
                let lit = Literal::usize_unsuffixed(*ident);
                quote_spanned!(span=> .#lit.mirror())
            }
            //PathComponent::IndexInt(idx) => quote_spanned!(span=> [#idx]),
            //PathComponent::IndexStr(s) => quote_spanned!(span=> [#s]),
            PathComponent::IndexInt(_) => quote_spanned!(span=> .sequence_get()),
            PathComponent::IndexStr(_) => quote_spanned!(span=> .map_get()),
        }
    }
}

impl FieldIdent {
    pub fn path_component_tokens(&self) -> proc_macro2::TokenStream {
        match self {
            FieldIdent::Named(s) => quote!(::keypath::internals::PathComponent::Named(#s)),
            FieldIdent::Unnamed(idx) => quote!(::keypath::internals::PathComponent::Unnamed(#idx)),
        }
    }
}
