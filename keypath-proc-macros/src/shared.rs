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

    /// The tokens generated to access the appropriate field or method on
    /// the underlying mirror type.
    pub fn mirror_item_access(&self, span: Span) -> proc_macro2::TokenStream {
        match self {
            PathComponent::Field(FieldIdent::Named(ident)) => {
                let ident = Ident::new(&ident, span);
                quote_spanned!(span=> .#ident)
            }
            PathComponent::Field(FieldIdent::Unnamed(ident)) => {
                let lit = Literal::usize_unsuffixed(*ident);
                quote_spanned!(span=> .#lit)
            }
            // NOTE: we tried generating index syntax to improve error diagnostics
            // but things got weird. Try again at some point?
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
