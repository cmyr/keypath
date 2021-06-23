use proc_macro::TokenStream;
use quote::quote;

use super::keypath_parse::{KeyPathMacroInput, SyntaxError};

pub(crate) fn keypath_impl(input: TokenStream) -> Result<TokenStream, SyntaxError> {
    //eprintln!("{:#?}", input);
    let KeyPathMacroInput { root, components } = KeyPathMacroInput::parse(input)?;

    let element_validators = components.iter().map(|comp| comp.traverse_type());
    let element_fields = components.iter().map(|comp| comp.to_tokens());
    let tokens = quote!(
        <#root as ::keypath::Keyable>::Mirror::new()
        #( #element_validators )*

        .to_key_path_with_root::<#root>(&[#( #element_fields ),*])
    );
    //eprintln!("{}", tokens);
    Ok(tokens.into())
}
