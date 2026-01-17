//! Script mode AST transformers.
//!
//! This module contains various AST transformation utilities for script mode,
//! such as generating extension traits for convenient method syntax.

use rustc_ast as ast;
use rustc_span::{Ident, Span, sym};

pub(crate) mod filter;
pub(crate) mod macros;
pub(crate) mod string;
pub(crate) mod truthy;
pub(crate) mod val;

#[allow(unused_imports)]
pub(crate) use filter::build_slice_helpers;
pub(crate) use macros::build_script_macros;
pub(crate) use string::build_string_helpers;
pub(crate) use truthy::build_truthy_helpers;
pub(crate) use val::{build_simple_ty, build_val_helpers};

/// Create #[allow(lint_name)] attribute for suppressing warnings
pub(crate) fn create_allow_attr(span: Span, lint_name: rustc_span::Symbol) -> ast::Attribute {
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
