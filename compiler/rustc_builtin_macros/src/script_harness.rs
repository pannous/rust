//! Generates a main function wrapper for script-mode files.
//!
//! When `-Z script` is enabled OR the file has a shebang (`#!`), this module
//! wraps bare statements/expressions in a generated `fn main()` if no main
//! function exists.

use rustc_ast as ast;
use rustc_ast::attr::contains_name;
use rustc_ast::entry::EntryPointType;
use rustc_expand::base::ResolverExpand;
use rustc_feature::Features;
use rustc_session::Session;
use rustc_session::config::Input;
use rustc_span::hygiene::AstPass;
use rustc_span::{DUMMY_SP, Ident, Span, sym};
use std::fs;
use thin_vec::ThinVec;

/// Inject a main function wrapper for script mode.
pub fn inject(
    krate: &mut ast::Crate,
    sess: &Session,
    _features: &Features,
    resolver: &mut dyn ResolverExpand,
) {
    // Activate if -Z script is enabled OR file has a shebang
    let script_mode = sess.opts.unstable_opts.script || has_shebang(&sess.io.input);
    if !script_mode {
        return;
    }

    // Check if file already has a main function or #[rustc_main]
    if has_entry_point(krate) {
        return;
    }

    // Check if there's anything to wrap
    if !has_executable_content(krate) {
        return;
    }

    // Set up expansion context for proper hygiene (like standard_library_imports does)
    let expn_id = resolver.expansion_for_ast_pass(
        DUMMY_SP,
        AstPass::ScriptMain,
        &[],
        None,
    );
    let def_site = DUMMY_SP.with_def_site_ctxt(expn_id.to_expn_id());
    let call_site = DUMMY_SP.with_call_site_ctxt(expn_id.to_expn_id());

    wrap_in_main(krate, def_site, call_site);
}

/// Check if the input source starts with a shebang (`#!`).
fn has_shebang(input: &Input) -> bool {
    match input {
        Input::File(path) => {
            // Read first few bytes of the file to check for shebang
            if let Ok(content) = fs::read_to_string(path) {
                is_shebang_line(&content)
            } else {
                false
            }
        }
        Input::Str { input, .. } => is_shebang_line(input),
    }
}

/// Check if the content starts with a shebang line.
/// A shebang is `#!` at the start, but NOT `#![` which is a Rust attribute.
fn is_shebang_line(content: &str) -> bool {
    if let Some(rest) = content.strip_prefix("#!") {
        // `#![` is a Rust inner attribute, not a shebang
        let next_char = rest.chars().next();
        next_char != Some('[')
    } else {
        false
    }
}

/// Check if the crate already has an entry point (main or #[rustc_main]).
fn has_entry_point(krate: &ast::Crate) -> bool {
    for item in &krate.items {
        if let ast::ItemKind::Fn(fn_) = &item.kind {
            let entry_type = rustc_ast::entry::entry_point_type(
                contains_name(&item.attrs, sym::rustc_main),
                true, // at_root
                Some(fn_.ident.name),
            );
            if matches!(entry_type, EntryPointType::MainNamed | EntryPointType::RustcMainAttr) {
                return true;
            }
        }
    }
    false
}

/// Check if the crate has content that should be wrapped in main.
/// Returns true if there are macro calls or other executable items.
fn has_executable_content(krate: &ast::Crate) -> bool {
    for item in &krate.items {
        if let ast::ItemKind::MacCall(_) = &item.kind {
            return true;
        }
    }
    false
}

/// Wrap executable items in a generated main function.
fn wrap_in_main(krate: &mut ast::Crate, def_site: Span, call_site: Span) {
    // Partition items: module-level items stay, macro calls go into main
    let (module_items, main_stmts) = partition_items(&krate.items);

    // Build items with proper hygiene contexts:
    // - def_site: for internal macro implementation (invisible to user)
    // - call_site: for macro names (visible to user code)
    // Don't call fully_expand_fragment - let normal expansion handle node IDs
    // (This follows the pattern from standard_library_imports.rs)
    let script_macros = build_script_macros(def_site, call_site);
    let main_fn = build_main(def_site, main_stmts);

    // Rebuild crate with script macros + module items + main function
    krate.items = script_macros;
    krate.items.extend(module_items);
    krate.items.push(main_fn);
}

/// Create #[allow(unused_macros)] attribute for suppressing warnings on auto-generated macros
fn create_allow_unused_attr(span: Span) -> ast::Attribute {
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
                TokenKind::Ident(sym::unused_macros, IdentIsRaw::No),
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

/// Build convenience macros for script mode: put! and eq!
/// - def_site: span for internal implementation (invisible to user)
/// - call_site: span for macro names (visible to user code)
fn build_script_macros(def_site: Span, call_site: Span) -> ThinVec<Box<ast::Item>> {
    use rustc_ast::token::{self, Delimiter, Lit, LitKind, TokenKind};
    use rustc_ast::tokenstream::{DelimSpacing, DelimSpan, Spacing, TokenStream, TokenTree};
    use rustc_span::Symbol;

    let mut items = ThinVec::new();

    // Create #[allow(unused_macros)] attribute for auto-generated macros
    let allow_unused = create_allow_unused_attr(def_site);

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


    // Helper to create a string literal token
    let str_lit = |s: &str| -> TokenTree {
        TokenTree::token_alone(
            TokenKind::Literal(Lit { kind: LitKind::Str, symbol: Symbol::intern(s), suffix: None }),
            def_site,
        )
    };

    // macro_rules! put {
    //     ($e:expr) => { println!("{}", $e) };           // put!(42) -> print with debug
    //     ($($arg:tt)*) => { println!($($arg)*) };       // put!("fmt", args) -> format string
    // }
    let put_body = vec![
        // First arm: ($e:expr) => { println!("{}", $e) };
        delim(Delimiter::Parenthesis, vec![
            TokenTree::token_alone(TokenKind::Dollar, def_site),
            ident("e"),
            TokenTree::token_alone(TokenKind::Colon, def_site),
            ident("expr"),
        ]),
        TokenTree::token_alone(TokenKind::FatArrow, def_site),
        delim(Delimiter::Brace, vec![
            ident("println"),
            TokenTree::token_alone(TokenKind::Bang, def_site),
            delim(Delimiter::Parenthesis, vec![
                str_lit("{}"),
                TokenTree::token_alone(TokenKind::Comma, def_site),
                TokenTree::token_alone(TokenKind::Dollar, def_site),
                ident("e"),
            ]),
        ]),
        TokenTree::token_alone(TokenKind::Semi, def_site),
        // Second arm: ($($arg:tt)*) => { println!($($arg)*) };
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
            ident("println"),
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
            ident("assert_eq"),
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

    // macro_rules! s { ($e:expr) => { $e.to_string() }; }
    // For converting string literals to String: s!("abc") -> "abc".to_string()
    let s_body = vec![
        // ($e:expr)
        delim(Delimiter::Parenthesis, vec![
            TokenTree::token_alone(TokenKind::Dollar, def_site),
            ident("e"),
            TokenTree::token_alone(TokenKind::Colon, def_site),
            ident("expr"),
        ]),
        TokenTree::token_alone(TokenKind::FatArrow, def_site),
        // { $e.to_string() }
        delim(Delimiter::Brace, vec![
            TokenTree::token_alone(TokenKind::Dollar, def_site),
            ident("e"),
            TokenTree::token_alone(TokenKind::Dot, def_site),
            ident("to_string"),
            delim(Delimiter::Parenthesis, vec![]),
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

    // macro_rules! __walrus { ($i:ident = $($e:tt)+) => { let mut $i = $($e)+; }; }
    // For Go-style short variable declarations: x := expr -> __walrus!(x = expr_tokens)
    // Uses `let mut` so variables are mutable by default (like Go/Python)
    let walrus_body = vec![
        // ($i:ident = $($e:tt)+)
        delim(Delimiter::Parenthesis, vec![
            TokenTree::token_alone(TokenKind::Dollar, def_site),
            ident("i"),
            TokenTree::token_alone(TokenKind::Colon, def_site),
            ident("ident"),
            TokenTree::token_alone(TokenKind::Eq, def_site),
            TokenTree::token_alone(TokenKind::Dollar, def_site),
            delim(Delimiter::Parenthesis, vec![
                TokenTree::token_alone(TokenKind::Dollar, def_site),
                ident("e"),
                TokenTree::token_alone(TokenKind::Colon, def_site),
                ident("tt"),
            ]),
            TokenTree::token_alone(TokenKind::Plus, def_site),
        ]),
        TokenTree::token_alone(TokenKind::FatArrow, def_site),
        // { let mut $i = $($e)+; }
        delim(Delimiter::Brace, vec![
            ident("let"),
            ident("mut"),
            TokenTree::token_alone(TokenKind::Dollar, def_site),
            ident("i"),
            TokenTree::token_alone(TokenKind::Eq, def_site),
            TokenTree::token_alone(TokenKind::Dollar, def_site),
            delim(Delimiter::Parenthesis, vec![
                TokenTree::token_alone(TokenKind::Dollar, def_site),
                ident("e"),
            ]),
            TokenTree::token_alone(TokenKind::Plus, def_site),
            TokenTree::token_alone(TokenKind::Semi, def_site),
        ]),
        TokenTree::token_alone(TokenKind::Semi, def_site),
    ];

    let walrus_macro = ast::MacroDef {
        body: Box::new(ast::DelimArgs {
            dspan: DelimSpan::from_single(def_site),
            delim: Delimiter::Brace,
            tokens: TokenStream::new(walrus_body),
        }),
        macro_rules: true,
        eii_extern_target: None,
    };

    items.push(Box::new(ast::Item {
        attrs: vec![allow_unused.clone()].into(),
        id: ast::DUMMY_NODE_ID,
        // Use call_site for the macro name so it's visible to user code
        kind: ast::ItemKind::MacroDef(Ident::new(sym::__walrus, call_site), walrus_macro),
        vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
        span: def_site,
        tokens: None,
    }));

    // macro_rules! __let { ($($t:tt)*) => { let $($t)*; }; }
    // For script-mode let statements with type annotations: `let x: Type = expr;`
    let let_body = vec![
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
        // { let $($t)*; }
        delim(Delimiter::Brace, vec![
            ident("let"),
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

    let let_macro = ast::MacroDef {
        body: Box::new(ast::DelimArgs {
            dspan: DelimSpan::from_single(def_site),
            delim: Delimiter::Brace,
            tokens: TokenStream::new(let_body),
        }),
        macro_rules: true,
        eii_extern_target: None,
    };

    items.push(Box::new(ast::Item {
        attrs: vec![allow_unused.clone()].into(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::ItemKind::MacroDef(Ident::new(sym::__let, call_site), let_macro),
        vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
        span: def_site,
        tokens: None,
    }));

    // macro_rules! __for { ($($t:tt)*) => { for $($t)* }; }
    let for_body = vec![
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
        delim(Delimiter::Brace, vec![
            ident("for"),
            TokenTree::token_alone(TokenKind::Dollar, def_site),
            delim(Delimiter::Parenthesis, vec![
                TokenTree::token_alone(TokenKind::Dollar, def_site),
                ident("t"),
            ]),
            TokenTree::token_alone(TokenKind::Star, def_site),
        ]),
        TokenTree::token_alone(TokenKind::Semi, def_site),
    ];

    let for_macro = ast::MacroDef {
        body: Box::new(ast::DelimArgs {
            dspan: DelimSpan::from_single(def_site),
            delim: Delimiter::Brace,
            tokens: TokenStream::new(for_body),
        }),
        macro_rules: true,
        eii_extern_target: None,
    };

    items.push(Box::new(ast::Item {
        attrs: vec![allow_unused.clone()].into(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::ItemKind::MacroDef(Ident::new(sym::__for, call_site), for_macro),
        vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
        span: def_site,
        tokens: None,
    }));

    // macro_rules! __while { ($($t:tt)*) => { while $($t)* }; }
    let while_body = vec![
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
        delim(Delimiter::Brace, vec![
            ident("while"),
            TokenTree::token_alone(TokenKind::Dollar, def_site),
            delim(Delimiter::Parenthesis, vec![
                TokenTree::token_alone(TokenKind::Dollar, def_site),
                ident("t"),
            ]),
            TokenTree::token_alone(TokenKind::Star, def_site),
        ]),
        TokenTree::token_alone(TokenKind::Semi, def_site),
    ];

    let while_macro = ast::MacroDef {
        body: Box::new(ast::DelimArgs {
            dspan: DelimSpan::from_single(def_site),
            delim: Delimiter::Brace,
            tokens: TokenStream::new(while_body),
        }),
        macro_rules: true,
        eii_extern_target: None,
    };

    items.push(Box::new(ast::Item {
        attrs: vec![allow_unused].into(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::ItemKind::MacroDef(Ident::new(sym::__while, call_site), while_macro),
        vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
        span: def_site,
        tokens: None,
    }));

    items
}

/// Partition items into module-level items and statements for main.
fn partition_items(
    items: &ThinVec<Box<ast::Item>>,
) -> (ThinVec<Box<ast::Item>>, ThinVec<ast::Stmt>) {
    let mut module_items = ThinVec::new();
    let mut main_stmts = ThinVec::new();

    for item in items {
        match &item.kind {
            // These must stay at module level
            ast::ItemKind::Use(_)
            | ast::ItemKind::ExternCrate(..)
            | ast::ItemKind::Mod(..)
            | ast::ItemKind::ForeignMod(_)
            | ast::ItemKind::GlobalAsm(_)
            | ast::ItemKind::TyAlias(_)
            | ast::ItemKind::Enum(..)
            | ast::ItemKind::Struct(..)
            | ast::ItemKind::Union(..)
            | ast::ItemKind::Trait(..)
            | ast::ItemKind::TraitAlias(..)
            | ast::ItemKind::Impl(_)
            | ast::ItemKind::Fn(_)
            | ast::ItemKind::MacroDef(..)
            | ast::ItemKind::Static(_)
            | ast::ItemKind::Const(_)
            | ast::ItemKind::Delegation(_)
            | ast::ItemKind::DelegationMac(_) => {
                module_items.push(item.clone());
            }

            // Macro calls become statements in main
            ast::ItemKind::MacCall(mac) => {
                let mac_stmt = ast::MacCallStmt {
                    mac: mac.clone(),
                    style: ast::MacStmtStyle::Semicolon,
                    attrs: item.attrs.clone(),
                    tokens: None,
                };
                main_stmts.push(ast::Stmt {
                    id: ast::DUMMY_NODE_ID,
                    kind: ast::StmtKind::MacCall(Box::new(mac_stmt)),
                    span: item.span,
                });
            }
        }
    }

    (module_items, main_stmts)
}

/// Build a `fn main() { <stmts> }` function.
fn build_main(span: Span, stmts: ThinVec<ast::Stmt>) -> Box<ast::Item> {
    use rustc_span::hygiene::SyntaxContext;
    // Use SyntaxContext::root() for the main name so entry point detection finds it
    let main_ident = Ident::new(sym::main, span.with_ctxt(SyntaxContext::root()));

    // Build empty return type ()
    let ret_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Tup(ThinVec::new()),
        span,
        tokens: None,
    });

    let decl = Box::new(ast::FnDecl {
        inputs: ThinVec::new(),
        output: ast::FnRetTy::Ty(ret_ty),
    });

    let sig = ast::FnSig {
        decl,
        header: ast::FnHeader::default(),
        span,
    };

    // Build block with statements
    let main_body = Box::new(ast::Block {
        stmts,
        id: ast::DUMMY_NODE_ID,
        rules: ast::BlockCheckMode::Default,
        span,
        tokens: None,
    });

    let main_fn = ast::ItemKind::Fn(Box::new(ast::Fn {
        defaultness: ast::Defaultness::Final,
        sig,
        ident: main_ident,
        generics: ast::Generics::default(),
        contract: None,
        body: Some(main_body),
        define_opaque: None,
        eii_impls: ThinVec::new(),
    }));

    // Node IDs will be assigned during macro expansion
    Box::new(ast::Item {
        attrs: ast::AttrVec::new(),
        id: ast::DUMMY_NODE_ID,
        kind: main_fn,
        vis: ast::Visibility { span, kind: ast::VisibilityKind::Public, tokens: None },
        span,
        tokens: None,
    })
}
