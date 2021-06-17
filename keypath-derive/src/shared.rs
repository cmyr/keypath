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

    pub fn validation_fn_name(&self) -> String {
        match self {
            PathComponent::Field(ident) => ident.validation_fn_name(),
            PathComponent::IndexInt(_) => "__keyable_index_int".into(),
            PathComponent::IndexStr(_) => "__keyable_index_str".into(),
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
