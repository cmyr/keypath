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
    let element_validators = path_elements.iter().map(PathElement::validator_fn_ident);
    let element_fields = path_elements
        .iter()
        .map(|element| element.element.to_field_tokens());
    let tokens = quote!(
        #root::fragment()
        #( .#element_validators() )*.to_key_path_with_root::<#root>(&[#( #element_fields ),*])
    );
    eprintln!("{}", tokens);
    Ok(tokens.into())
}

struct PathElement {
    element: FieldIdent,
    span: Span,
}

impl PathElement {
    fn validator_fn_ident(&self) -> proc_macro2::Ident {
        let ident = self.element.validation_fn_name();
        proc_macro2::Ident::new(&ident.to_string(), self.span.into())
    }
}

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

fn try_append_components(
    iter: &mut TokenIter,
    elements: &mut Vec<PathElement>,
) -> Result<(), SyntaxError> {
    let token = next_token(iter)?;
    let span = token.span();
    match &token {
        TokenTree::Literal(lit) => {
            match litrs::Literal::from(lit.clone()) {
                litrs::Literal::Integer(int) => {
                    let element = int
                        .value::<usize>()
                        .map(FieldIdent::Unnamed)
                        .ok_or_else(|| syntax(token, "indexes must be unsigned integers"))?;
                    elements.push(PathElement { element, span });
                }
                // a path like "This.hi.0.2" will have "0.2" parsed as a float literal
                litrs::Literal::Float(float)
                    if float.type_suffix().is_none() && float.exponent_part().is_empty() =>
                {
                    let first = float
                        .integer_part()
                        .parse::<usize>()
                        .map(FieldIdent::Unnamed)
                        .map_err(|_| syntax(&token, "indexes must be unsigned integers"))?;
                    let second = float
                        .fractional_part()
                        .ok_or_else(|| syntax(&token, "paths should not have trailing periods"))?
                        .parse::<usize>()
                        .map(FieldIdent::Unnamed)
                        .map_err(|_| syntax(token, "indexes must be unsigned integers"))?;
                    elements.push(PathElement {
                        element: first,
                        span,
                    });
                    elements.push(PathElement {
                        element: second,
                        span,
                    });
                }
                other => {
                    return Err(syntax(
                        token,
                        format!("expected index or field name, found literal '{}'", other),
                    ))
                }
            };
            Ok(())
        }
        TokenTree::Ident(ident) => {
            elements.push(PathElement {
                element: FieldIdent::Named(ident.to_string()),
                span,
            });
            Ok(())
        }
        other => Err(syntax(other, "expected ident")),
    }
}

/// Ok(true) when more work to do, Ok(false) when done
fn next_path_element(
    iter: &mut TokenIter,
    elements: &mut Vec<PathElement>,
) -> Result<bool, SyntaxError> {
    match iter.next() {
        None => return Ok(false),
        Some(TokenTree::Punct(p)) if p.as_char() == '.' && p.spacing() == Spacing::Alone => (),
        Some(token) => return Err(syntax(token, "expected '.'")),
    };

    try_append_components(iter, elements)?;
    Ok(true)
}

fn collect_path_elements(iter: &mut TokenIter) -> Result<Vec<PathElement>, SyntaxError> {
    let mut result = Vec::new();
    loop {
        if !next_path_element(iter, &mut result)? {
            return Ok(result)
        }
    }
}
