//! Script mode convenience macros: put!, printf!, eq!, s!, typeid!
//!
//! Generates macro_rules! definitions for common script operations.
//!
//! Note: Some functions are currently unused as their functionality
//! has been migrated to compiler/extensions/src/all.rs

use rustc_ast as ast;
use rustc_ast::token::{self, Delimiter, Lit, LitKind, TokenKind};
use rustc_ast::tokenstream::{DelimSpacing, DelimSpan, Spacing, TokenStream, TokenTree};
use rustc_span::{Ident, Span, Symbol, sym};
use thin_vec::ThinVec;

use super::create_allow_attr;

/// Build convenience macros for script mode: put!, printf!, eq!, s!, typeid!
/// - def_site: span for internal implementation (invisible to user)
/// - call_site: span for macro names (visible to user code)
pub fn build_script_macros(def_site: Span, call_site: Span) -> ThinVec<Box<ast::Item>> {
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
        eii_declaration: None,
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
        eii_declaration: None,
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
        eii_declaration: None,
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

    // macro_rules! eqs { ($left:expr, $right:expr) => { assert_eq!(format!("{:?}", $left), $right) }; }
    // String equality macro: compares Debug representation of left to string literal right
    let eqs_body = vec![
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
        // { assert_eq!(format!("{:?}", $left), $right) }
        delim(Delimiter::Brace, vec![
            ident_user("assert_eq"),
            TokenTree::token_alone(TokenKind::Bang, def_site),
            delim(Delimiter::Parenthesis, vec![
                ident_user("format"),
                TokenTree::token_alone(TokenKind::Bang, def_site),
                delim(Delimiter::Parenthesis, vec![
                    TokenTree::token_alone(TokenKind::Literal(
                        token::Lit::new(token::LitKind::Str, sym::empty_braces_debug, None)
                    ), def_site),
                    TokenTree::token_alone(TokenKind::Comma, def_site),
                    TokenTree::token_alone(TokenKind::Dollar, def_site),
                    ident("left"),
                ]),
                TokenTree::token_alone(TokenKind::Comma, def_site),
                TokenTree::token_alone(TokenKind::Dollar, def_site),
                ident("right"),
            ]),
        ]),
        TokenTree::token_alone(TokenKind::Semi, def_site),
    ];

    let eqs_macro = ast::MacroDef {
        body: Box::new(ast::DelimArgs {
            dspan: DelimSpan::from_single(def_site),
            delim: Delimiter::Brace,
            tokens: TokenStream::new(eqs_body),
        }),
        macro_rules: true,
        eii_declaration: None,
    };

    items.push(Box::new(ast::Item {
        attrs: vec![allow_unused.clone()].into(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::ItemKind::MacroDef(Ident::new(sym::eqs, call_site), eqs_macro),
        vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
        span: def_site,
        tokens: None,
    }));

    // macro_rules! seq { ($left:expr, $right:expr) => { assert!(slice_eq(&$left, &$right)) }; }
    // Slice equality macro for comparing arrays with Vecs
    let seq_body = vec![
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
        // { assert!(slice_eq(&$left, &$right)) }
        delim(Delimiter::Brace, vec![
            ident_user("assert"),
            TokenTree::token_alone(TokenKind::Bang, def_site),
            delim(Delimiter::Parenthesis, vec![
                ident_user("slice_eq"),
                delim(Delimiter::Parenthesis, vec![
                    TokenTree::token_alone(TokenKind::And, def_site),
                    TokenTree::token_alone(TokenKind::Dollar, def_site),
                    ident("left"),
                    TokenTree::token_alone(TokenKind::Comma, def_site),
                    TokenTree::token_alone(TokenKind::And, def_site),
                    TokenTree::token_alone(TokenKind::Dollar, def_site),
                    ident("right"),
                ]),
            ]),
        ]),
        TokenTree::token_alone(TokenKind::Semi, def_site),
    ];

    let seq_macro = ast::MacroDef {
        body: Box::new(ast::DelimArgs {
            dspan: DelimSpan::from_single(def_site),
            delim: Delimiter::Brace,
            tokens: TokenStream::new(seq_body),
        }),
        macro_rules: true,
        eii_declaration: None,
    };

    items.push(Box::new(ast::Item {
        attrs: vec![allow_unused.clone()].into(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::ItemKind::MacroDef(Ident::new(sym::seq, call_site), seq_macro),
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
        eii_declaration: None,
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
        eii_declaration: None,
    };

    items.push(Box::new(ast::Item {
        attrs: vec![allow_unused.clone()].into(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::ItemKind::MacroDef(Ident::new(sym::typeid, call_site), typeid_macro),
        vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
        span: def_site,
        tokens: None,
    }));

    // macro_rules! exit {
    //     () => { exit(0) };
    //     ($code:expr) => { exit($code) };
    // }
    // Exit the process with optional exit code (default 0)
    let exit_body = vec![
        // First arm: () => { exit(0) };
        delim(Delimiter::Parenthesis, vec![]),
        TokenTree::token_alone(TokenKind::FatArrow, def_site),
        delim(Delimiter::Brace, vec![
            ident_user("exit"),
            delim(Delimiter::Parenthesis, vec![
                TokenTree::token_alone(
                    TokenKind::Literal(Lit { kind: LitKind::Integer, symbol: sym::integer(0), suffix: None }),
                    def_site,
                ),
            ]),
        ]),
        TokenTree::token_alone(TokenKind::Semi, def_site),
        // Second arm: ($code:expr) => { exit($code) };
        delim(Delimiter::Parenthesis, vec![
            TokenTree::token_alone(TokenKind::Dollar, def_site),
            ident("code"),
            TokenTree::token_alone(TokenKind::Colon, def_site),
            ident("expr"),
        ]),
        TokenTree::token_alone(TokenKind::FatArrow, def_site),
        delim(Delimiter::Brace, vec![
            ident_user("exit"),
            delim(Delimiter::Parenthesis, vec![
                TokenTree::token_alone(TokenKind::Dollar, def_site),
                ident("code"),
            ]),
        ]),
        TokenTree::token_alone(TokenKind::Semi, def_site),
    ];

    let exit_macro = ast::MacroDef {
        body: Box::new(ast::DelimArgs {
            dspan: DelimSpan::from_single(def_site),
            delim: Delimiter::Brace,
            tokens: TokenStream::new(exit_body),
        }),
        macro_rules: true,
        eii_declaration: None,
    };

    items.push(Box::new(ast::Item {
        attrs: vec![allow_unused.clone()].into(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::ItemKind::MacroDef(Ident::new(sym::exit, call_site), exit_macro),
        vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
        span: def_site,
        tokens: None,
    }));

    // macro_rules! __if {
    //     ($cond:expr ; $($rest:tt)*) => { if (&$cond).is_truthy() $($rest)* };
    // }
    // For script-mode if statements with truthy semantics
    let if_body = vec![
        // ($cond:expr ; $($rest:tt)*)
        delim(Delimiter::Parenthesis, vec![
            TokenTree::token_alone(TokenKind::Dollar, def_site),
            ident("cond"),
            TokenTree::token_alone(TokenKind::Colon, def_site),
            ident("expr"),
            TokenTree::token_alone(TokenKind::Semi, def_site),
            TokenTree::token_alone(TokenKind::Dollar, def_site),
            delim(Delimiter::Parenthesis, vec![
                TokenTree::token_alone(TokenKind::Dollar, def_site),
                ident("rest"),
                TokenTree::token_alone(TokenKind::Colon, def_site),
                ident("tt"),
            ]),
            TokenTree::token_alone(TokenKind::Star, def_site),
        ]),
        TokenTree::token_alone(TokenKind::FatArrow, def_site),
        // { if (&$cond).is_truthy() $($rest)* }
        delim(Delimiter::Brace, vec![
            ident("if"),
            delim(Delimiter::Parenthesis, vec![
                TokenTree::token_alone(TokenKind::And, def_site),
                TokenTree::token_alone(TokenKind::Dollar, def_site),
                ident("cond"),
            ]),
            TokenTree::token_alone(TokenKind::Dot, def_site),
            ident_user("is_truthy"),
            delim(Delimiter::Parenthesis, vec![]),
            TokenTree::token_alone(TokenKind::Dollar, def_site),
            delim(Delimiter::Parenthesis, vec![
                TokenTree::token_alone(TokenKind::Dollar, def_site),
                ident("rest"),
            ]),
            TokenTree::token_alone(TokenKind::Star, def_site),
        ]),
        TokenTree::token_alone(TokenKind::Semi, def_site),
    ];

    let if_macro = ast::MacroDef {
        body: Box::new(ast::DelimArgs {
            dspan: DelimSpan::from_single(def_site),
            delim: Delimiter::Brace,
            tokens: TokenStream::new(if_body),
        }),
        macro_rules: true,
        eii_declaration: None,
    };

    items.push(Box::new(ast::Item {
        attrs: vec![allow_unused.clone()].into(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::ItemKind::MacroDef(Ident::new(sym::__if, call_site), if_macro),
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
        eii_declaration: None,
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

/// Build the `exit` function for script mode:
/// ```ignore
/// fn exit(code: i32) -> ! {
///     std::process::exit(code)
/// }
/// ```
fn build_exit_function(def_site: Span, call_site: Span) -> Box<ast::Item> {
    let allow_dead_code = create_allow_attr(def_site, sym::dead_code);

    // Parameter: code: i32
    let code_param = ast::Param {
        attrs: ThinVec::new(),
        ty: Box::new(ast::Ty {
            id: ast::DUMMY_NODE_ID,
            kind: ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(sym::i32, call_site))),
            span: call_site,
            tokens: None,
        }),
        pat: Box::new(ast::Pat {
            id: ast::DUMMY_NODE_ID,
            kind: ast::PatKind::Ident(
                ast::BindingMode::NONE,
                Ident::new(sym::code, call_site),
                None,
            ),
            span: call_site,
            tokens: None,
        }),
        id: ast::DUMMY_NODE_ID,
        span: call_site,
        is_placeholder: false,
    };

    // Return type: ! (never type)
    let ret_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Never,
        span: call_site,
        tokens: None,
    });

    let fn_sig = ast::FnSig {
        decl: Box::new(ast::FnDecl {
            inputs: ThinVec::from([code_param]),
            output: ast::FnRetTy::Ty(ret_ty),
        }),
        header: ast::FnHeader::default(),
        span: def_site,
    };

    // Body: std::process::exit(code)
    let std_process_exit_path = ast::Path {
        span: call_site,
        segments: ThinVec::from([
            ast::PathSegment::from_ident(Ident::new(sym::std, call_site)),
            ast::PathSegment::from_ident(Ident::new(sym::process, call_site)),
            ast::PathSegment::from_ident(Ident::new(sym::exit, call_site)),
        ]),
        tokens: None,
    };

    let code_arg = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::new(sym::code, call_site))),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let exit_call = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Call(
            Box::new(ast::Expr {
                id: ast::DUMMY_NODE_ID,
                kind: ast::ExprKind::Path(None, std_process_exit_path),
                span: call_site,
                attrs: ThinVec::new(),
                tokens: None,
            }),
            ThinVec::from([code_arg]),
        ),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let body_block = Box::new(ast::Block {
        stmts: ThinVec::from([ast::Stmt {
            id: ast::DUMMY_NODE_ID,
            kind: ast::StmtKind::Expr(exit_call),
            span: def_site,
        }]),
        id: ast::DUMMY_NODE_ID,
        rules: ast::BlockCheckMode::Default,
        span: def_site,
        tokens: None,
    });

    let fn_def = ast::Fn {
        defaultness: ast::Defaultness::Final,
        ident: Ident::new(sym::exit, call_site),
        generics: ast::Generics::default(),
        sig: fn_sig,
        contract: None,
        body: Some(body_block),
        define_opaque: None,
        eii_impls: ThinVec::new(),
    };

    Box::new(ast::Item {
        attrs: vec![allow_dead_code].into(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::ItemKind::Fn(Box::new(fn_def)),
        vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
        span: def_site,
        tokens: None,
    })
}

/// Build the `approx_eq` function for script mode:
/// ```ignore
/// fn approx_eq(a: f64, b: f64) -> bool {
///     (a - b).abs() < 1e-9_f64.max(a.abs() * 1e-9).max(b.abs() * 1e-9)
/// }
/// ```
/// Uses relative epsilon for better precision with varying magnitudes.
fn build_approx_eq_function(def_site: Span, call_site: Span) -> Box<ast::Item> {
    let allow_dead_code = create_allow_attr(def_site, sym::dead_code);

    // Parameter a: f64
    let a_param = ast::Param {
        attrs: ThinVec::new(),
        ty: Box::new(ast::Ty {
            id: ast::DUMMY_NODE_ID,
            kind: ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(sym::f64, call_site))),
            span: call_site,
            tokens: None,
        }),
        pat: Box::new(ast::Pat {
            id: ast::DUMMY_NODE_ID,
            kind: ast::PatKind::Ident(
                ast::BindingMode::NONE,
                Ident::new(sym::a, call_site),
                None,
            ),
            span: call_site,
            tokens: None,
        }),
        id: ast::DUMMY_NODE_ID,
        span: call_site,
        is_placeholder: false,
    };

    // Parameter b: f64
    let b_param = ast::Param {
        attrs: ThinVec::new(),
        ty: Box::new(ast::Ty {
            id: ast::DUMMY_NODE_ID,
            kind: ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(sym::f64, call_site))),
            span: call_site,
            tokens: None,
        }),
        pat: Box::new(ast::Pat {
            id: ast::DUMMY_NODE_ID,
            kind: ast::PatKind::Ident(
                ast::BindingMode::NONE,
                Ident::new(sym::b, call_site),
                None,
            ),
            span: call_site,
            tokens: None,
        }),
        id: ast::DUMMY_NODE_ID,
        span: call_site,
        is_placeholder: false,
    };

    // Return type: bool
    let ret_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(sym::bool, call_site))),
        span: call_site,
        tokens: None,
    });

    let fn_sig = ast::FnSig {
        decl: Box::new(ast::FnDecl {
            inputs: ThinVec::from([a_param, b_param]),
            output: ast::FnRetTy::Ty(ret_ty),
        }),
        header: ast::FnHeader::default(),
        span: def_site,
    };

    // Body: (a - b).abs() < 1e-9_f64.max(a.abs() * 1e-9).max(b.abs() * 1e-9)
    // Build: a - b
    let a_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::new(sym::a, call_site))),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let b_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::new(sym::b, call_site))),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // a - b
    let diff = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Binary(
            ast::BinOp { node: ast::BinOpKind::Sub, span: call_site },
            a_expr.clone(),
            b_expr.clone(),
        ),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // (a - b).abs()
    let diff_abs = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::abs, call_site)),
            receiver: diff,
            args: ThinVec::new(),
            span: call_site,
        })),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // 1e-9 literal
    let epsilon = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Lit(token::Lit {
            kind: token::LitKind::Float,
            symbol: sym::float_1e_minus_9,
            suffix: Some(sym::f64),
        }),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // a.abs()
    let a_abs = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::abs, call_site)),
            receiver: a_expr.clone(),
            args: ThinVec::new(),
            span: call_site,
        })),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // a.abs() * 1e-9
    let a_rel_eps = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Binary(
            ast::BinOp { node: ast::BinOpKind::Mul, span: call_site },
            a_abs,
            epsilon.clone(),
        ),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // b.abs()
    let b_abs = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::abs, call_site)),
            receiver: b_expr.clone(),
            args: ThinVec::new(),
            span: call_site,
        })),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // b.abs() * 1e-9
    let b_rel_eps = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Binary(
            ast::BinOp { node: ast::BinOpKind::Mul, span: call_site },
            b_abs,
            epsilon.clone(),
        ),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // 1e-9.max(a.abs() * 1e-9)
    let eps_max_a = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::max, call_site)),
            receiver: epsilon,
            args: ThinVec::from([a_rel_eps]),
            span: call_site,
        })),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // .max(b.abs() * 1e-9)
    let tolerance = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::max, call_site)),
            receiver: eps_max_a,
            args: ThinVec::from([b_rel_eps]),
            span: call_site,
        })),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // (a - b).abs() < tolerance
    let comparison = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Binary(
            ast::BinOp { node: ast::BinOpKind::Lt, span: call_site },
            diff_abs,
            tolerance,
        ),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let body_block = Box::new(ast::Block {
        stmts: ThinVec::from([ast::Stmt {
            id: ast::DUMMY_NODE_ID,
            kind: ast::StmtKind::Expr(comparison),
            span: def_site,
        }]),
        id: ast::DUMMY_NODE_ID,
        rules: ast::BlockCheckMode::Default,
        span: def_site,
        tokens: None,
    });

    let fn_def = ast::Fn {
        defaultness: ast::Defaultness::Final,
        ident: Ident::new(sym::approx_eq, call_site),
        generics: ast::Generics::default(),
        sig: fn_sig,
        contract: None,
        body: Some(body_block),
        define_opaque: None,
        eii_impls: ThinVec::new(),
    };

    Box::new(ast::Item {
        attrs: vec![allow_dead_code].into(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::ItemKind::Fn(Box::new(fn_def)),
        vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
        span: def_site,
        tokens: None,
    })
}

/// Build math constants for script mode: τ, π
fn build_math_constants(def_site: Span, call_site: Span) -> ThinVec<Box<ast::Item>> {
    let allow_dead_code = create_allow_attr(def_site, sym::dead_code);
    let mut items = ThinVec::new();

    // const τ: f64 = std::f64::consts::TAU;
    // const π: f64 = std::f64::consts::PI;

    let make_const = |name: Symbol, const_name: Symbol| -> Box<ast::Item> {
        // std::f64::consts::CONST_NAME
        let const_path = ast::Path {
            span: call_site,
            segments: ThinVec::from([
                ast::PathSegment::from_ident(Ident::new(sym::std, call_site)),
                ast::PathSegment::from_ident(Ident::new(sym::f64, call_site)),
                ast::PathSegment::from_ident(Ident::new(sym::consts, call_site)),
                ast::PathSegment::from_ident(Ident::new(const_name, call_site)),
            ]),
            tokens: None,
        };

        let const_expr = Box::new(ast::Expr {
            id: ast::DUMMY_NODE_ID,
            kind: ast::ExprKind::Path(None, const_path),
            span: call_site,
            attrs: ThinVec::new(),
            tokens: None,
        });

        let const_ty = Box::new(ast::Ty {
            id: ast::DUMMY_NODE_ID,
            kind: ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(sym::f64, call_site))),
            span: call_site,
            tokens: None,
        });

        Box::new(ast::Item {
            attrs: vec![allow_dead_code.clone()].into(),
            id: ast::DUMMY_NODE_ID,
            kind: ast::ItemKind::Const(Box::new(ast::ConstItem {
                defaultness: ast::Defaultness::Final,
                ident: Ident::new(name, call_site),
                generics: ast::Generics::default(),
                ty: const_ty,
                rhs: Some(ast::ConstItemRhs::Body(const_expr)),
                define_opaque: None,
            })),
            vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
            span: def_site,
            tokens: None,
        })
    };

    items.push(make_const(sym::tau, sym::TAU));
    items.push(make_const(sym::pi, sym::PI));
    // Also add the Greek letter versions
    items.push(make_const(sym::tau_greek, sym::TAU));
    items.push(make_const(sym::pi_greek, sym::PI));

    items
}
