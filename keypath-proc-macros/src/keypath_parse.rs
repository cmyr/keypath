use std::iter::{FromIterator, Peekable};

use proc_macro::token_stream::IntoIter as StreamIter;
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use quote::quote_spanned;

use super::shared::PathComponent;

pub(crate) struct KeyPathMacroInput {
    pub(crate) root: proc_macro2::TokenStream,
    pub(crate) components: Vec<SpannedComponent>,
}

pub(crate) struct SpannedComponent {
    element: PathComponent,
    span: Span,
}

pub(crate) struct SyntaxError {
    message: String,
    span: Span,
}

enum FieldLiteral {
    Named(String),
    Unnamed(usize),
    UnnamedPair(usize, usize),
}

type TokenIter = Peekable<StreamIter>;

impl SpannedComponent {
    pub(crate) fn traverse_type(&self) -> proc_macro2::TokenStream {
        self.element.mirror_item_access(self.span.into())
    }

    pub(crate) fn to_tokens(&self) -> proc_macro2::TokenStream {
        let tokens = self.element.path_component_tokens();
        let span = self.span.into();
        quote_spanned!(span=> #tokens)
    }
}

impl KeyPathMacroInput {
    pub(crate) fn parse(input: TokenStream) -> Result<Self, SyntaxError> {
        let mut iter = input.into_iter().peekable();
        //next_token(&mut iter);

        let root = expect_root(&mut iter)?.into();
        let components = collect_path_components(&mut iter)?;
        Ok(KeyPathMacroInput { root, components })
    }
}

fn expect_root(iter: &mut TokenIter) -> Result<TokenStream, SyntaxError> {
    let mut result = Vec::new();
    let root = next_token(iter)?;
    match &root {
        TokenTree::Ident(_) => result.push(root),
        _other => {
            return Err(SyntaxError::new(
                root.span(),
                "Keypath should start with Type",
            ))
        }
    }

    if matches!(iter.peek(), Some(&TokenTree::Punct(ref p)) if p.as_char() == '<') {
        result.extend(expect_root_generics(iter)?);
    }
    Ok(result.into_iter().collect())
}

fn expect_root_generics(iter: &mut TokenIter) -> Result<Vec<TokenTree>, SyntaxError> {
    let mut result = Vec::new();
    result.push(TokenTree::Punct(expect_punct(iter, '<')?));
    let mut done = false;
    for token in iter {
        done = matches!(token, TokenTree::Punct(ref p) if p.as_char() == '>');
        result.push(token);
        if done {
            break;
        }
    }
    if !done {
        Err(SyntaxError::new(
            result.first().unwrap().span(),
            "Missing closing '>'",
        ))
    } else {
        Ok(result)
    }
}

fn collect_path_components(iter: &mut TokenIter) -> Result<Vec<SpannedComponent>, SyntaxError> {
    let mut result = Vec::new();
    loop {
        match iter.next() {
            None => return Ok(result),
            Some(TokenTree::Punct(p)) if p.as_char() == '.' => match iter
                .next()
                .ok_or_else(|| SyntaxError::new(p.span(), "'.' must be followed by a field"))?
            {
                TokenTree::Ident(ident) => result.push(SpannedComponent {
                    span: ident.span(),
                    element: PathComponent::named(ident.to_string()),
                }),
                TokenTree::Literal(lit) => append_fields_from_lit(&lit, &mut result)?,
                other => {
                    return Err(SyntaxError::new(
                        other.span(),
                        format!("expected field identifier, found '{}'", other),
                    ))
                }
            },
            Some(TokenTree::Group(g)) if matches!(g.delimiter(), Delimiter::Bracket) => {
                result.push(expect_index(&g)?);
            }
            Some(other) => {
                eprintln!("BAD TOKEN {:?}", other);
                return Err(SyntaxError::new(other.span(), "expected '.' or '['"));
            }
        }
    }
}

fn append_fields_from_lit(
    lit: &Literal,
    result: &mut Vec<SpannedComponent>,
) -> Result<(), SyntaxError> {
    let span = lit.span();
    match parse_literal(lit)? {
        FieldLiteral::Named(name) => result.push(SpannedComponent {
            element: PathComponent::named(name),
            span,
        }),
        FieldLiteral::Unnamed(idx) => result.push(SpannedComponent {
            element: PathComponent::unnamed(idx),
            span,
        }),
        FieldLiteral::UnnamedPair(first, second) => {
            result.push(SpannedComponent {
                element: PathComponent::unnamed(first),
                span,
            });
            result.push(SpannedComponent {
                element: PathComponent::unnamed(second),
                span,
            });
        }
    };
    Ok(())
}

fn expect_index(g: &Group) -> Result<SpannedComponent, SyntaxError> {
    let mut tokens = g.stream().into_iter();
    let lit = match tokens.next() {
        Some(TokenTree::Literal(lit)) => lit,
        _ => {
            return Err(SyntaxError::new(
                g.span(),
                "Brackets must contain a string or integer literal",
            ))
        }
    };
    match parse_literal(&lit)? {
        FieldLiteral::Named(name) => Ok(SpannedComponent {
            element: PathComponent::IndexStr(name),
            span: lit.span(),
        }),
        FieldLiteral::Unnamed(idx) => Ok(SpannedComponent {
            element: PathComponent::IndexInt(idx),
            span: lit.span(),
        }),
        FieldLiteral::UnnamedPair(..) => Err(SyntaxError::new(
            lit.span(),
            "collection indices must be string or unsigned integer literals",
        )),
    }
}

fn parse_literal(lit: &Literal) -> Result<FieldLiteral, SyntaxError> {
    let raw_lit = lit.to_string();
    if raw_lit.starts_with('"') {
        return Ok(FieldLiteral::Named(raw_lit.trim_matches('"').to_string()));
    }

    if let Ok(idx) = raw_lit.parse::<usize>() {
        return Ok(FieldLiteral::Unnamed(idx));
    }

    // see if it's a float, where both sides of the decimal place are valid usizes
    raw_lit
        .split_once('.')
        .and_then(|(front, back)| {
            let front = front.parse::<usize>().ok()?;
            let back = back.parse::<usize>().ok()?;
            Some(FieldLiteral::UnnamedPair(front, back))
        })
        .ok_or_else(|| SyntaxError::new(lit.span(), "identifiers must be strings or integers"))
}

fn next_token(iter: &mut TokenIter) -> Result<TokenTree, SyntaxError> {
    iter.next().ok_or_else(|| SyntaxError {
        message: "unexpected end of input".to_owned(),
        span: Span::call_site(),
    })
}

fn expect_punct(iter: &mut TokenIter, chr: char) -> Result<Punct, SyntaxError> {
    match next_token(iter)? {
        TokenTree::Punct(p) if p.as_char() == chr => Ok(p),
        other => Err(SyntaxError::new(
            other.span(),
            format!("expected '{}', found '{}'", chr, other),
        )),
    }
}

impl SyntaxError {
    fn new(span: Span, message: impl std::fmt::Display) -> Self {
        SyntaxError {
            message: message.to_string(),
            span,
        }
    }

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
