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
