//! derive macros for the keypath crate.

#![deny(clippy::trivially_copy_pass_by_ref)]

extern crate proc_macro;

mod attr;
mod keyable;
mod keypath;
mod shared;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_derive(Keyable)]
pub fn derive_keyable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    keyable::derive_keyable_impl(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Create a strongly-typed `KeyPath`.
///
/// This verifies at compile-time that the path is valid.
///
/// This macro expects a *type name*, followed by one or more *path components*.
/// Path components may be either *fields* or *indices*.
///
/// - field: a single '`.`' character, followed by either a valid identifier or
///   a single unsized integer.
/// - indicies: a pair of brackets (`[]`) containing either a string literal or
///   an unsized integer.
///
/// Fields should correspond to named or unnamed fields on the base type.
/// Indicies refer to members of collections.
///
/// # Examples
///
/// The following are *semantically* valid keypaths. (Their actual validity
/// would depend on these fields existing in the underlying types.)
///
/// ```no_compile
/// keypath!(Person.profile.name);
/// keypath!(Element.size.0);
/// keypath!(Person.friends[10].name);
/// keypath!(Person.friends["常羽辰"].address);
/// keypath!(Thing.field.0["friends"].count);
/// keypath!(Thing.1[2][3].size.width);
/// ```
#[proc_macro]
pub fn keypath(input: TokenStream) -> TokenStream {
    match keypath::keypath_impl(input) {
        Ok(expanded) => expanded,
        Err(error) => error.into_compile_error(),
    }
}
