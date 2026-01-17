//! Script mode convenience macros: put!, printf!, eq!, s!, typeid!
//!
//! Generates macro_rules! definitions for common script operations.

use rustc_ast as ast;
use rustc_ast::token::{self, Delimiter, Lit, LitKind, TokenKind};
use rustc_ast::tokenstream::{DelimSpacing, DelimSpan, Spacing, TokenStream, TokenTree};
use rustc_span::{Ident, Span, Symbol, sym};
use thin_vec::ThinVec;

use super::create_allow_attr;

/// Build convenience macros for script mode: put!, printf!, eq!, s!, typeid!
/// - def_site: span for internal implementation (invisible to user)
/// - call_site: span for macro names (visible to user code)
pub(crate) fn build_script_macros(def_site: Span, call_site: Span) -> ThinVec<Box<ast::Item>> {
    let mut items = ThinVec::new();

    // Create #[allow(unused_macros)] attribute for auto-generated macros
    let allow_unused = create_allow_attr(def_site, sym::unused_macros);

    // Helper to create a delimited group (uses def_site for internal implementation)
    let delim = |d: Delimiter, inner: Vec<TokenTree>| -> TokenTree {
        TokenTree::Delimited(
            DelimSpan::from_single(def_site),
            DelimSpacing::new(Spacing::Alone, Spacing::Alone),
            d,
            TokenStream::new(inner),
        )
    };

    // Helper to create an identifier token (uses def_site for internal implementation)
    let ident = |s: &str| -> TokenTree {
        TokenTree::token_alone(TokenKind::Ident(Symbol::intern(s), token::IdentIsRaw::No), def_site)
    };

    // Helper to create an identifier that resolves in user's scope (for prelude macros)
    let ident_user = |s: &str| -> TokenTree {
        TokenTree::token_alone(TokenKind::Ident(Symbol::intern(s), token::IdentIsRaw::No), call_site)
    };


    // Helper to create a string literal token
    let str_lit = |s: &str| -> TokenTree {
        TokenTree::token_alone(
            TokenKind::Literal(Lit { kind: LitKind::Str, symbol: Symbol::intern(s), suffix: None }),
            def_site,
        )
    };

    // macro_rules! put {
    //     () => { println!() };                                           // put!() -> newline
    //     ($e:expr $(,)?) => { println!("{:?}", $e) };                     // put!(42) -> single expr (Debug)
    //     ($first:expr, $($rest:expr),+ $(,)?) => {                        // put!(a, b, c) -> Python style
    //         print!("{:?}", $first);
    //         $(print!(" {:?}", $rest);)+
    //         println!();
    //     };
    // }
    // Uses {:?} (Debug) format for broader type support (Option, Vec, etc.)
    let put_body = vec![
        // First arm: () => { println!() };
        delim(Delimiter::Parenthesis, vec![]),
        TokenTree::token_alone(TokenKind::FatArrow, def_site),
        delim(Delimiter::Brace, vec![
            ident_user("println"),
            TokenTree::token_alone(TokenKind::Bang, def_site),
            delim(Delimiter::Parenthesis, vec![]),
        ]),
        TokenTree::token_alone(TokenKind::Semi, def_site),
        // Second arm: ($e:expr $(,)?) => { println!("{:?}", $e) };
        delim(Delimiter::Parenthesis, vec![
            TokenTree::token_alone(TokenKind::Dollar, def_site),
            ident("e"),
            TokenTree::token_alone(TokenKind::Colon, def_site),
            ident("expr"),
            // $(,)?
            TokenTree::token_alone(TokenKind::Dollar, def_site),
            delim(Delimiter::Parenthesis, vec![
                TokenTree::token_alone(TokenKind::Comma, def_site),
            ]),
            TokenTree::token_alone(TokenKind::Question, def_site),
        ]),
        TokenTree::token_alone(TokenKind::FatArrow, def_site),
        delim(Delimiter::Brace, vec![
            ident_user("println"),
            TokenTree::token_alone(TokenKind::Bang, def_site),
            delim(Delimiter::Parenthesis, vec![
                str_lit("{:?}"),
                TokenTree::token_alone(TokenKind::Comma, def_site),
                TokenTree::token_alone(TokenKind::Dollar, def_site),
                ident("e"),
            ]),
        ]),
        TokenTree::token_alone(TokenKind::Semi, def_site),
        // Third arm: ($first:expr, $($rest:expr),+ $(,)?) => { print!("{:?}", $first); $(print!(" {:?}", $rest);)+ println!(); };
        delim(Delimiter::Parenthesis, vec![
            // $first:expr
            TokenTree::token_alone(TokenKind::Dollar, def_site),
            ident("first"),
            TokenTree::token_alone(TokenKind::Colon, def_site),
            ident("expr"),
            TokenTree::token_alone(TokenKind::Comma, def_site),
            // $($rest:expr),+
            TokenTree::token_alone(TokenKind::Dollar, def_site),
            delim(Delimiter::Parenthesis, vec![
                TokenTree::token_alone(TokenKind::Dollar, def_site),
                ident("rest"),
                TokenTree::token_alone(TokenKind::Colon, def_site),
                ident("expr"),
            ]),
            TokenTree::token_alone(TokenKind::Comma, def_site),
            TokenTree::token_alone(TokenKind::Plus, def_site),
            // $(,)?
            TokenTree::token_alone(TokenKind::Dollar, def_site),
            delim(Delimiter::Parenthesis, vec![
                TokenTree::token_alone(TokenKind::Comma, def_site),
            ]),
            TokenTree::token_alone(TokenKind::Question, def_site),
        ]),
        TokenTree::token_alone(TokenKind::FatArrow, def_site),
        delim(Delimiter::Brace, vec![
            // print!("{:?}", $first);
            ident_user("print"),
            TokenTree::token_alone(TokenKind::Bang, def_site),
            delim(Delimiter::Parenthesis, vec![
                str_lit("{:?}"),
                TokenTree::token_alone(TokenKind::Comma, def_site),
                TokenTree::token_alone(TokenKind::Dollar, def_site),
                ident("first"),
            ]),
            TokenTree::token_alone(TokenKind::Semi, def_site),
            // $(print!(" {:?}", $rest);)+
            TokenTree::token_alone(TokenKind::Dollar, def_site),
            delim(Delimiter::Parenthesis, vec![
                ident_user("print"),
                TokenTree::token_alone(TokenKind::Bang, def_site),
                delim(Delimiter::Parenthesis, vec![
                    str_lit(" {:?}"),
                    TokenTree::token_alone(TokenKind::Comma, def_site),
                    TokenTree::token_alone(TokenKind::Dollar, def_site),
                    ident("rest"),
                ]),
                TokenTree::token_alone(TokenKind::Semi, def_site),
            ]),
            TokenTree::token_alone(TokenKind::Plus, def_site),
            // println!();
            ident_user("println"),
            TokenTree::token_alone(TokenKind::Bang, def_site),
            delim(Delimiter::Parenthesis, vec![]),
            TokenTree::token_alone(TokenKind::Semi, def_site),
        ]),
        TokenTree::token_alone(TokenKind::Semi, def_site),
    ];

    let put_macro = ast::MacroDef {
        body: Box::new(ast::DelimArgs {
            dspan: DelimSpan::from_single(def_site),
            delim: Delimiter::Brace,
            tokens: TokenStream::new(put_body),
        }),
        macro_rules: true,
        eii_extern_target: None,
    };

    items.push(Box::new(ast::Item {
        attrs: vec![allow_unused.clone()].into(),
        id: ast::DUMMY_NODE_ID,
        // Use call_site for the macro name so it's visible to user code
        kind: ast::ItemKind::MacroDef(Ident::new(sym::put, call_site), put_macro),
        vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
        span: def_site,
        tokens: None,
    }));

    // macro_rules! printf { ($($arg:tt)*) => { println!($($arg)*) }; }
    // Format string version of put - passes through to println!
    let printf_body = vec![
        // ($($arg:tt)*) => { println!($($arg)*) };
        delim(Delimiter::Parenthesis, vec![
            TokenTree::token_alone(TokenKind::Dollar, def_site),
            delim(Delimiter::Parenthesis, vec![
                TokenTree::token_alone(TokenKind::Dollar, def_site),
                ident("arg"),
                TokenTree::token_alone(TokenKind::Colon, def_site),
                ident("tt"),
            ]),
            TokenTree::token_alone(TokenKind::Star, def_site),
        ]),
        TokenTree::token_alone(TokenKind::FatArrow, def_site),
        delim(Delimiter::Brace, vec![
            ident_user("println"),
            TokenTree::token_alone(TokenKind::Bang, def_site),
            delim(Delimiter::Parenthesis, vec![
                TokenTree::token_alone(TokenKind::Dollar, def_site),
                delim(Delimiter::Parenthesis, vec![
                    TokenTree::token_alone(TokenKind::Dollar, def_site),
                    ident("arg"),
                ]),
                TokenTree::token_alone(TokenKind::Star, def_site),
            ]),
        ]),
        TokenTree::token_alone(TokenKind::Semi, def_site),
    ];

    let printf_macro = ast::MacroDef {
        body: Box::new(ast::DelimArgs {
            dspan: DelimSpan::from_single(def_site),
            delim: Delimiter::Brace,
            tokens: TokenStream::new(printf_body),
        }),
        macro_rules: true,
        eii_extern_target: None,
    };

    items.push(Box::new(ast::Item {
        attrs: vec![allow_unused.clone()].into(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::ItemKind::MacroDef(Ident::new(sym::printf, call_site), printf_macro),
        vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
        span: def_site,
        tokens: None,
    }));

    // macro_rules! eq { ($left:expr, $right:expr) => { assert_eq!($left, $right) }; }
    let eq_body = vec![
        // ($left:expr, $right:expr)
        delim(Delimiter::Parenthesis, vec![
            TokenTree::token_alone(TokenKind::Dollar, def_site),
            ident("left"),
            TokenTree::token_alone(TokenKind::Colon, def_site),
            ident("expr"),
            TokenTree::token_alone(TokenKind::Comma, def_site),
            TokenTree::token_alone(TokenKind::Dollar, def_site),
            ident("right"),
            TokenTree::token_alone(TokenKind::Colon, def_site),
            ident("expr"),
        ]),
        TokenTree::token_alone(TokenKind::FatArrow, def_site),
        // { assert_eq!($left, $right) }
        delim(Delimiter::Brace, vec![
            ident_user("assert_eq"),
            TokenTree::token_alone(TokenKind::Bang, def_site),
            delim(Delimiter::Parenthesis, vec![
                TokenTree::token_alone(TokenKind::Dollar, def_site),
                ident("left"),
                TokenTree::token_alone(TokenKind::Comma, def_site),
                TokenTree::token_alone(TokenKind::Dollar, def_site),
                ident("right"),
            ]),
        ]),
        TokenTree::token_alone(TokenKind::Semi, def_site),
    ];

    let eq_macro = ast::MacroDef {
        body: Box::new(ast::DelimArgs {
            dspan: DelimSpan::from_single(def_site),
            delim: Delimiter::Brace,
            tokens: TokenStream::new(eq_body),
        }),
        macro_rules: true,
        eii_extern_target: None,
    };

    items.push(Box::new(ast::Item {
        attrs: vec![allow_unused.clone()].into(),
        id: ast::DUMMY_NODE_ID,
        // Use call_site for the macro name so it's visible to user code
        kind: ast::ItemKind::MacroDef(Ident::new(sym::eq, call_site), eq_macro),
        vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
        span: def_site,
        tokens: None,
    }));

    // macro_rules! s { ($e:expr) => { { let __s: String = $e.into(); __s } }; }
    // For converting string literals to String: s!("abc") -> "abc".into()
    // Uses .into() with type annotation for reliable conversion
    // Note: String and into use call_site so they resolve in user's scope (where prelude is available)
    let s_body = vec![
        // ($e:expr)
        delim(Delimiter::Parenthesis, vec![
            TokenTree::token_alone(TokenKind::Dollar, def_site),
            ident("e"),
            TokenTree::token_alone(TokenKind::Colon, def_site),
            ident("expr"),
        ]),
        TokenTree::token_alone(TokenKind::FatArrow, def_site),
        // { { let __s: String = $e.into(); __s } }
        delim(Delimiter::Brace, vec![
            delim(Delimiter::Brace, vec![
                ident("let"),
                ident("__s"),
                TokenTree::token_alone(TokenKind::Colon, def_site),
                // String with call_site hygiene to resolve in user's namespace
                TokenTree::token_alone(TokenKind::Ident(sym::String, token::IdentIsRaw::No), call_site),
                TokenTree::token_alone(TokenKind::Eq, def_site),
                TokenTree::token_alone(TokenKind::Dollar, def_site),
                ident("e"),
                TokenTree::token_alone(TokenKind::Dot, call_site),
                // into with call_site to resolve in user's namespace
                TokenTree::token_alone(TokenKind::Ident(sym::into, token::IdentIsRaw::No), call_site),
                delim(Delimiter::Parenthesis, vec![]),
                TokenTree::token_alone(TokenKind::Semi, def_site),
                ident("__s"),
            ]),
        ]),
        TokenTree::token_alone(TokenKind::Semi, def_site),
    ];

    let s_macro = ast::MacroDef {
        body: Box::new(ast::DelimArgs {
            dspan: DelimSpan::from_single(def_site),
            delim: Delimiter::Brace,
            tokens: TokenStream::new(s_body),
        }),
        macro_rules: true,
        eii_extern_target: None,
    };

    items.push(Box::new(ast::Item {
        attrs: vec![allow_unused.clone()].into(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::ItemKind::MacroDef(Ident::new(sym::s, call_site), s_macro),
        vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
        span: def_site,
        tokens: None,
    }));

    // macro_rules! typeid { ($e:expr) => { std::any::type_name_of_val(&$e) }; }
    // Get the type name of an expression at runtime
    let typeid_body = vec![
        // ($e:expr)
        delim(Delimiter::Parenthesis, vec![
            TokenTree::token_alone(TokenKind::Dollar, def_site),
            ident("e"),
            TokenTree::token_alone(TokenKind::Colon, def_site),
            ident("expr"),
        ]),
        TokenTree::token_alone(TokenKind::FatArrow, def_site),
        // { std::any::type_name_of_val(&$e) }
        delim(Delimiter::Brace, vec![
            ident_user("std"),
            TokenTree::token_alone(TokenKind::PathSep, call_site),
            ident_user("any"),
            TokenTree::token_alone(TokenKind::PathSep, call_site),
            ident_user("type_name_of_val"),
            delim(Delimiter::Parenthesis, vec![
                TokenTree::token_alone(TokenKind::And, def_site),
                TokenTree::token_alone(TokenKind::Dollar, def_site),
                ident("e"),
            ]),
        ]),
        TokenTree::token_alone(TokenKind::Semi, def_site),
    ];

    let typeid_macro = ast::MacroDef {
        body: Box::new(ast::DelimArgs {
            dspan: DelimSpan::from_single(def_site),
            delim: Delimiter::Brace,
            tokens: TokenStream::new(typeid_body),
        }),
        macro_rules: true,
        eii_extern_target: None,
    };

    items.push(Box::new(ast::Item {
        attrs: vec![allow_unused.clone()].into(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::ItemKind::MacroDef(Ident::new(sym::typeid, call_site), typeid_macro),
        vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
        span: def_site,
        tokens: None,
    }));

    // macro_rules! __stmt { ($($t:tt)*) => { $($t)*; }; }
    // For script-mode statements parsed as block
    let stmt_body = vec![
        // ($($t:tt)*)
        delim(Delimiter::Parenthesis, vec![
            TokenTree::token_alone(TokenKind::Dollar, def_site),
            delim(Delimiter::Parenthesis, vec![
                TokenTree::token_alone(TokenKind::Dollar, def_site),
                ident("t"),
                TokenTree::token_alone(TokenKind::Colon, def_site),
                ident("tt"),
            ]),
            TokenTree::token_alone(TokenKind::Star, def_site),
        ]),
        TokenTree::token_alone(TokenKind::FatArrow, def_site),
        // { $($t)*; }
        delim(Delimiter::Brace, vec![
            TokenTree::token_alone(TokenKind::Dollar, def_site),
            delim(Delimiter::Parenthesis, vec![
                TokenTree::token_alone(TokenKind::Dollar, def_site),
                ident("t"),
            ]),
            TokenTree::token_alone(TokenKind::Star, def_site),
            TokenTree::token_alone(TokenKind::Semi, def_site),
        ]),
        TokenTree::token_alone(TokenKind::Semi, def_site),
    ];

    let stmt_macro = ast::MacroDef {
        body: Box::new(ast::DelimArgs {
            dspan: DelimSpan::from_single(def_site),
            delim: Delimiter::Brace,
            tokens: TokenStream::new(stmt_body),
        }),
        macro_rules: true,
        eii_extern_target: None,
    };

    items.push(Box::new(ast::Item {
        attrs: vec![allow_unused].into(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::ItemKind::MacroDef(Ident::new(sym::__stmt, call_site), stmt_macro),
        vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
        span: def_site,
        tokens: None,
    }));

    items
}
