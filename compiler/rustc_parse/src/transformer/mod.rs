//! Script mode AST transformers.
//!
//! This module contains AST transformation utilities for script mode.
//! Most functionality is now in compiler/extensions/src/all.rs which
//! gets parsed and injected via parse_extensions().

use rustc_ast as ast;
use rustc_span::{Ident, Span, sym};

mod extensions;
#[allow(dead_code)]
mod macros;
#[allow(dead_code)]
mod val;

// Unused modules kept for reference (can be deleted later):
// mod filter;
// mod slice;
// mod string;
// mod truthy;

pub use extensions::parse_extensions;
pub use macros::build_script_macros;
pub use val::build_simple_ty;

/// Create #[allow(lint_name)] attribute for suppressing warnings
pub fn create_allow_attr(span: Span, lint_name: rustc_span::Symbol) -> ast::Attribute {
    use rustc_ast::{AttrArgs, AttrItemKind, AttrKind, AttrStyle, NormalAttr, Path, PathSegment, Safety};

    let path = Path {
        span,
        segments: vec![
            PathSegment::from_ident(Ident::new(sym::allow, span)),
        ]
        .into(),
        tokens: None,
    };

    let args = AttrArgs::Delimited(ast::DelimArgs {
        dspan: rustc_ast::tokenstream::DelimSpan::from_single(span),
        delim: rustc_ast::token::Delimiter::Parenthesis,
        tokens: {
            use rustc_ast::token::{IdentIsRaw, TokenKind};
            use rustc_ast::tokenstream::{TokenStream, TokenTree};
            TokenStream::new(vec![TokenTree::token_alone(
                TokenKind::Ident(lint_name, IdentIsRaw::No),
                span,
            )])
        },
    });

    ast::Attribute {
        kind: AttrKind::Normal(Box::new(NormalAttr {
            item: ast::AttrItem {
                unsafety: Safety::Default,
                path,
                args: AttrItemKind::Unparsed(args),
                tokens: None
            },
            tokens: None
        })),
        id: ast::AttrId::from_u32(0),
        style: AttrStyle::Outer,
        span,
    }
}

/// Create #[derive(Trait1, Trait2, ...)] attribute
pub fn create_derive_attr(span: Span, traits: &[rustc_span::Symbol]) -> ast::Attribute {
    use rustc_ast::{AttrArgs, AttrItemKind, AttrKind, AttrStyle, NormalAttr, Path, PathSegment, Safety};
    use rustc_ast::token::{IdentIsRaw, TokenKind};
    use rustc_ast::tokenstream::{TokenStream, TokenTree};

    let path = Path {
        span,
        segments: vec![PathSegment::from_ident(Ident::new(sym::derive, span))].into(),
        tokens: None,
    };

    let mut tokens = Vec::new();
    for (i, &trait_sym) in traits.iter().enumerate() {
        if i > 0 {
            tokens.push(TokenTree::token_alone(TokenKind::Comma, span));
        }
        tokens.push(TokenTree::token_alone(TokenKind::Ident(trait_sym, IdentIsRaw::No), span));
    }

    let args = AttrArgs::Delimited(ast::DelimArgs {
        dspan: ast::tokenstream::DelimSpan::from_single(span),
        delim: ast::token::Delimiter::Parenthesis,
        tokens: TokenStream::new(tokens),
    });

    ast::Attribute {
        kind: AttrKind::Normal(Box::new(NormalAttr {
            item: ast::AttrItem {
                unsafety: Safety::Default,
                path,
                args: AttrItemKind::Unparsed(args),
                tokens: None,
            },
            tokens: None,
        })),
        id: ast::AttrId::from_u32(0),
        style: AttrStyle::Outer,
        span,
    }
}
