use proc_macro2::{Ident, Literal, Span};
use quote::quote;

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
            PathComponent::IndexInt(idx) => quote!(::keypath::PathComponent::IndexInt(#idx)),
            PathComponent::IndexStr(s) => quote!(::keypath::PathComponent::IndexStr(#s)),
        }
    }

    //pub fn validation_fn_name(&self) -> String {
        //match self {
            //PathComponent::Field(ident) => ident.validation_fn_name(),
            //PathComponent::IndexInt(_) => "__keyable_index_int".into(),
            //PathComponent::IndexStr(_) => "__keyable_index_str".into(),
        //}
    //}

    pub fn to_tokens(&self) -> proc_macro2::TokenStream {
        match self {
            PathComponent::Field(FieldIdent::Named(ident)) => {
                let ident = Ident::new(&ident, Span::call_site());
                quote!(#ident.get())
            }
            PathComponent::Field(FieldIdent::Unnamed(ident)) => {
                let lit = Literal::usize_unsuffixed(*ident);
                quote!(#lit.get())
            }
            //PathComponent::IndexInt(idx) => quote!([#idx]),
            //PathComponent::IndexStr(s) => quote!([#s]),
            PathComponent::IndexInt(_) => quote!(__keyable_index_int()),
            PathComponent::IndexStr(_) => quote!(__keyable_index_str()),
        }
    }
}

impl FieldIdent {
    pub fn validation_fn_name(&self) -> String {
        match self {
            FieldIdent::Named(s) => s.clone(),
            FieldIdent::Unnamed(idx) => format!("_{}", idx),
        }
    }

    pub fn path_component_tokens(&self) -> proc_macro2::TokenStream {
        match self {
            FieldIdent::Named(s) => quote!(::keypath::PathComponent::Named(#s)),
            FieldIdent::Unnamed(idx) => quote!(::keypath::PathComponent::Unnamed(#idx)),
        }
    }
}
