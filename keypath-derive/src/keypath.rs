use std::borrow::Borrow;
use std::fmt::Display;
use std::iter::FromIterator;

use proc_macro::token_stream::IntoIter as TokenIter;
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use quote::quote;

use super::attr::FieldIdent;

pub(crate) fn keypath_impl(input: TokenStream) -> Result<TokenStream, SyntaxError> {
    //eprintln!("{}", input);
    let mut iter = input.into_iter();
    let root: TokenStream = TokenTree::Ident(require_ident(&mut iter)?).into();
    let root: proc_macro2::TokenStream = root.into();
    let path_elements = collect_path_elements(&mut iter)?;
    let element_validators = path_elements
        .iter()
        .map(|element| element.element.validation_fn_name());
    let element_fields = path_elements
        .iter()
        .map(|element| element.element.to_field_tokens());
    let tokens = quote!(
        #root::fragment()
        #( .#element_validators() )*.to_key_path_with_root::<#root>(&[#( #element_fields ),*])
        //let fields = ;
        //validated_value.to_key_path<#root>(fields)
    );
    //eprintln!("{}", &tokens);
    Ok(tokens.into())
    //quote!()
    //for token in iter {
    //eprintln!("{:?}", token);
    //}
    //Ok(TokenTree::Ident(root).into())
}

struct PathElement {
    element: FieldIdent,
    // use me in errors somehow?
    #[allow(dead_code)]
    span: Span,
}

//enum Element {
//Ord(usize),
//Field(String),
//}

pub(crate) struct SyntaxError {
    message: String,
    span: Span,
}

impl SyntaxError {
    pub(crate) fn into_compile_error(self) -> TokenStream {
        // compile_error! { $message }
        TokenStream::from_iter(vec![
            TokenTree::Ident(Ident::new("compile_error", self.span)),
            TokenTree::Punct({
                let mut punct = Punct::new('!', Spacing::Alone);
                punct.set_span(self.span);
                punct
            }),
            TokenTree::Group({
                let mut group = Group::new(Delimiter::Brace, {
                    TokenStream::from_iter(vec![TokenTree::Literal({
                        let mut string = Literal::string(&self.message);
                        string.set_span(self.span);
                        string
                    })])
                });
                group.set_span(self.span);
                group
            }),
        ])
    }
}

fn next_token(iter: &mut TokenIter) -> Result<TokenTree, SyntaxError> {
    iter.next().ok_or_else(|| SyntaxError {
        message: "unexpected end of input".to_owned(),
        span: Span::call_site(),
    })
}

fn syntax<T: Borrow<TokenTree>, M: Display>(token: T, message: M) -> SyntaxError {
    SyntaxError {
        message: message.to_string(),
        span: token.borrow().span(),
    }
}

fn require_ident(iter: &mut TokenIter) -> Result<Ident, SyntaxError> {
    match next_token(iter)? {
        TokenTree::Ident(ident) => Ok(ident),
        other => Err(syntax(other, "expected ident")),
    }
}

fn require_path_component(iter: &mut TokenIter) -> Result<PathElement, SyntaxError> {
    let token = next_token(iter)?;
    let span = token.span();
    match &token {
        TokenTree::Literal(lit) => {
            let element = match litrs::Literal::from(lit.clone()) {
                litrs::Literal::Integer(int) => int
                    .value::<usize>()
                    .map(FieldIdent::Unnamed)
                    .ok_or_else(|| syntax(token, "indexes must be unsigned integers")),
                other => Err(syntax(
                    token,
                    format!("expected index or field name, found literal '{}'", other),
                )),
            }?;
            Ok(PathElement { element, span })
        }
        TokenTree::Ident(ident) => Ok(PathElement {
            element: FieldIdent::Named(ident.to_string()),
            span,
        }),
        other => Err(syntax(other, "expected ident")),
    }
}

fn next_path_element(iter: &mut TokenIter) -> Result<Option<PathElement>, SyntaxError> {
    match iter.next() {
        None => return Ok(None),
        Some(TokenTree::Punct(p)) if p.as_char() == '.' && p.spacing() == Spacing::Alone => p,
        Some(token) => return Err(syntax(token, "expected '.'")),
    };

    let component = require_path_component(iter)?;
    Ok(Some(component))
}

fn collect_path_elements(iter: &mut TokenIter) -> Result<Vec<PathElement>, SyntaxError> {
    let mut result = Vec::new();
    loop {
        match next_path_element(iter)? {
            Some(element) => result.push(element),
            None => break Ok(result),
        }
    }
}
