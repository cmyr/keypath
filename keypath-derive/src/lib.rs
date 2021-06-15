//! derive macros for the keypath crate.

#![deny(clippy::trivially_copy_pass_by_ref)]

extern crate proc_macro;

mod attr;
mod keyable;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_derive(Keyable)]
pub fn derive_keyable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    keyable::derive_keyable_impl(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
