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
use rustc_span::{DUMMY_SP, Ident, Span, kw, sym};
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
    let string_helpers = build_string_helpers(def_site, call_site);
    let truthy_helpers = build_truthy_helpers(def_site, call_site);
    let main_fn = build_main(def_site, main_stmts);

    // Rebuild crate with script macros + helpers + module items + main function
    krate.items = script_macros;
    krate.items.extend(string_helpers);
    krate.items.extend(truthy_helpers);
    krate.items.extend(module_items);
    krate.items.push(main_fn);
}

/// Create #[allow(lint_name)] attribute for suppressing warnings
fn create_allow_attr(span: Span, lint_name: rustc_span::Symbol) -> ast::Attribute {
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

/// Build convenience macros for script mode: put! and eq!
/// - def_site: span for internal implementation (invisible to user)
/// - call_site: span for macro names (visible to user code)
fn build_script_macros(def_site: Span, call_site: Span) -> ThinVec<Box<ast::Item>> {
    use rustc_ast::token::{self, Delimiter, Lit, LitKind, TokenKind};
    use rustc_ast::tokenstream::{DelimSpacing, DelimSpan, Spacing, TokenStream, TokenTree};
    use rustc_span::Symbol;

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
            ident_user("println"),
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
        attrs: vec![allow_unused.clone()].into(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::ItemKind::MacroDef(Ident::new(sym::__while, call_site), while_macro),
        vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
        span: def_site,
        tokens: None,
    }));

    // macro_rules! __if {
    //     ($cond:expr ; $($rest:tt)*) => { if Truthy::is_truthy(&$cond) $($rest)* };
    // }
    // Simple pattern: condition followed by semicolon, then the rest passes through
    let if_body = vec![
        // Single arm: ($cond:expr ; $($rest:tt)*) => { if Truthy::is_truthy(&$cond) $($rest)* }
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
        delim(Delimiter::Brace, vec![
            // if (&$cond).is_truthy() $($rest)*
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
        eii_extern_target: None,
    };

    items.push(Box::new(ast::Item {
        attrs: vec![allow_unused].into(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::ItemKind::MacroDef(Ident::new(sym::__if, call_site), if_macro),
        vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
        span: def_site,
        tokens: None,
    }));

    items
}

/// Build helper trait and impl for string methods: first(), last(), size(), length()
/// Generates:
/// ```ignore
/// trait ScriptStrExt {
///     fn first(&self) -> &str;
///     fn last(&self) -> &str;
///     fn size(&self) -> usize;
///     fn length(&self) -> usize;
/// }
/// impl ScriptStrExt for &str { ... }
/// ```
fn build_string_helpers(def_site: Span, call_site: Span) -> ThinVec<Box<ast::Item>> {
    let mut items = ThinVec::new();

    // Create #[allow(dead_code)] attribute
    let allow_dead_code = create_allow_attr(def_site, sym::dead_code);

    // Symbol for the trait name
    let trait_name = sym::ScriptStrExt;

    // Build trait method signatures - use call_site so they're visible to user code
    let trait_items = build_str_ext_trait_items(call_site);

    // Build the trait definition
    let trait_def = ast::Trait {
        constness: ast::Const::No,
        safety: ast::Safety::Default,
        is_auto: ast::IsAuto::No,
        ident: Ident::new(trait_name, call_site),
        generics: ast::Generics::default(),
        bounds: Vec::new(),
        items: trait_items,
    };

    items.push(Box::new(ast::Item {
        attrs: vec![allow_dead_code.clone()].into(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::ItemKind::Trait(Box::new(trait_def)),
        vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
        span: def_site,
        tokens: None,
    }));

    // Build the impl block for &str
    let impl_items = build_str_ext_impl_items(def_site, call_site);

    // Build &str type
    let str_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Ref(
            None,
            ast::MutTy {
                ty: Box::new(ast::Ty {
                    id: ast::DUMMY_NODE_ID,
                    kind: ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(sym::str, call_site))),
                    span: call_site,
                    tokens: None,
                }),
                mutbl: ast::Mutability::Not,
            },
        ),
        span: call_site,
        tokens: None,
    });

    // Build trait reference path
    let trait_path = ast::Path::from_ident(Ident::new(trait_name, call_site));

    let impl_def = ast::Impl {
        generics: ast::Generics::default(),
        constness: ast::Const::No,
        of_trait: Some(Box::new(ast::TraitImplHeader {
            defaultness: ast::Defaultness::Final,
            safety: ast::Safety::Default,
            polarity: ast::ImplPolarity::Positive,
            trait_ref: ast::TraitRef { path: trait_path, ref_id: ast::DUMMY_NODE_ID },
        })),
        self_ty: str_ty,
        items: impl_items,
    };

    items.push(Box::new(ast::Item {
        attrs: ThinVec::new(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::ItemKind::Impl(impl_def),
        vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
        span: def_site,
        tokens: None,
    }));

    items
}

/// Build Truthy trait and implementations for common types.
/// Generates:
/// ```ignore
/// trait Truthy {
///     fn is_truthy(&self) -> bool;
/// }
/// impl Truthy for bool { fn is_truthy(&self) -> bool { *self } }
/// impl Truthy for i32 { fn is_truthy(&self) -> bool { *self != 0 } }
/// // ... etc
/// ```
fn build_truthy_helpers(def_site: Span, call_site: Span) -> ThinVec<Box<ast::Item>> {
    let mut items = ThinVec::new();

    // Create #[allow(dead_code)] attribute
    let allow_dead_code = create_allow_attr(def_site, sym::dead_code);

    // Build the Truthy trait definition
    let trait_name = sym::Truthy;

    // Build trait method signature: fn is_truthy(&self) -> bool
    let trait_items = build_truthy_trait_items(call_site);

    let trait_def = ast::Trait {
        constness: ast::Const::No,
        safety: ast::Safety::Default,
        is_auto: ast::IsAuto::No,
        ident: Ident::new(trait_name, call_site),
        generics: ast::Generics::default(),
        bounds: Vec::new(),
        items: trait_items,
    };

    items.push(Box::new(ast::Item {
        attrs: vec![allow_dead_code.clone()].into(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::ItemKind::Trait(Box::new(trait_def)),
        vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
        span: def_site,
        tokens: None,
    }));

    // Add implementations for various types
    let trait_path = ast::Path::from_ident(Ident::new(trait_name, call_site));

    // impl Truthy for bool
    items.push(build_truthy_impl_bool(def_site, call_site, &trait_path));

    // impl Truthy for integer types
    for ty_name in &[sym::i8, sym::i16, sym::i32, sym::i64, sym::i128, sym::isize,
                     sym::u8, sym::u16, sym::u32, sym::u64, sym::u128, sym::usize] {
        items.push(build_truthy_impl_integer(def_site, call_site, &trait_path, *ty_name));
    }

    // impl Truthy for float types
    for ty_name in &[sym::f32, sym::f64] {
        items.push(build_truthy_impl_float(def_site, call_site, &trait_path, *ty_name));
    }

    // impl Truthy for &str
    items.push(build_truthy_impl_str_ref(def_site, call_site, &trait_path));

    // impl Truthy for String
    items.push(build_truthy_impl_string(def_site, call_site, &trait_path));

    items
}

/// Build the Truthy trait method signature: fn is_truthy(&self) -> bool
fn build_truthy_trait_items(span: Span) -> ThinVec<Box<ast::AssocItem>> {
    let mut items = ThinVec::new();

    let fn_sig = ast::FnSig {
        decl: Box::new(ast::FnDecl {
            inputs: ThinVec::from([ast::Param {
                attrs: ThinVec::new(),
                ty: Box::new(ast::Ty {
                    id: ast::DUMMY_NODE_ID,
                    kind: ast::TyKind::Ref(
                        None,  // no explicit lifetime
                        ast::MutTy {
                            ty: Box::new(ast::Ty {
                                id: ast::DUMMY_NODE_ID,
                                kind: ast::TyKind::ImplicitSelf,
                                span,
                                tokens: None,
                            }),
                            mutbl: ast::Mutability::Not,
                        },
                    ),
                    span,
                    tokens: None,
                }),
                pat: Box::new(ast::Pat {
                    id: ast::DUMMY_NODE_ID,
                    kind: ast::PatKind::Ident(
                        ast::BindingMode::NONE,
                        Ident::with_dummy_span(kw::SelfLower).with_span_pos(span),
                        None,
                    ),
                    span,
                    tokens: None,
                }),
                id: ast::DUMMY_NODE_ID,
                span,
                is_placeholder: false,
            }]),
            output: ast::FnRetTy::Ty(Box::new(ast::Ty {
                id: ast::DUMMY_NODE_ID,
                kind: ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(sym::bool, span))),
                span,
                tokens: None,
            })),
        }),
        header: ast::FnHeader::default(),
        span,
    };

    let fn_def = ast::Fn {
        defaultness: ast::Defaultness::Final,
        ident: Ident::new(sym::is_truthy, span),
        generics: ast::Generics::default(),
        sig: fn_sig,
        contract: None,
        body: None, // No body for trait method signature
        define_opaque: None,
        eii_impls: ThinVec::new(),
    };

    items.push(Box::new(ast::Item {
        attrs: ThinVec::new(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::AssocItemKind::Fn(Box::new(fn_def)),
        vis: ast::Visibility { span, kind: ast::VisibilityKind::Inherited, tokens: None },
        span,
        tokens: None,
    }));

    items
}

/// Build impl Truthy for bool { fn is_truthy(&self) -> bool { *self } }
fn build_truthy_impl_bool(
    def_site: Span,
    call_site: Span,
    trait_path: &ast::Path,
) -> Box<ast::Item> {
    // Body: *self
    let body_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Unary(
            ast::UnOp::Deref,
            Box::new(ast::Expr {
                id: ast::DUMMY_NODE_ID,
                kind: ast::ExprKind::Path(None, ast::Path::from_ident(
                    Ident::with_dummy_span(kw::SelfLower).with_span_pos(call_site)
                )),
                span: call_site,
                attrs: ThinVec::new(),
                tokens: None,
            }),
        ),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    build_truthy_impl_for_type(
        def_site,
        call_site,
        trait_path,
        sym::bool,
        body_expr,
    )
}

/// Build impl Truthy for integer { fn is_truthy(&self) -> bool { *self != 0 } }
fn build_truthy_impl_integer(
    def_site: Span,
    call_site: Span,
    trait_path: &ast::Path,
    ty_name: rustc_span::Symbol,
) -> Box<ast::Item> {
    // Body: *self != 0
    let deref_self = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Unary(
            ast::UnOp::Deref,
            Box::new(ast::Expr {
                id: ast::DUMMY_NODE_ID,
                kind: ast::ExprKind::Path(None, ast::Path::from_ident(
                    Ident::with_dummy_span(kw::SelfLower).with_span_pos(call_site)
                )),
                span: call_site,
                attrs: ThinVec::new(),
                tokens: None,
            }),
        ),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let zero_lit = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Lit(rustc_ast::token::Lit {
            kind: rustc_ast::token::LitKind::Integer,
            symbol: sym::integer(0),
            suffix: None,
        }),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let body_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Binary(
            rustc_span::source_map::Spanned {
                node: ast::BinOpKind::Ne,
                span: call_site,
            },
            deref_self,
            zero_lit,
        ),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    build_truthy_impl_for_type(def_site, call_site, trait_path, ty_name, body_expr)
}

/// Build impl Truthy for float { fn is_truthy(&self) -> bool { *self != 0.0 } }
fn build_truthy_impl_float(
    def_site: Span,
    call_site: Span,
    trait_path: &ast::Path,
    ty_name: rustc_span::Symbol,
) -> Box<ast::Item> {
    // Body: *self != 0.0
    let deref_self = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Unary(
            ast::UnOp::Deref,
            Box::new(ast::Expr {
                id: ast::DUMMY_NODE_ID,
                kind: ast::ExprKind::Path(None, ast::Path::from_ident(
                    Ident::with_dummy_span(kw::SelfLower).with_span_pos(call_site)
                )),
                span: call_site,
                attrs: ThinVec::new(),
                tokens: None,
            }),
        ),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // 0.0 literal
    let zero_lit = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Lit(rustc_ast::token::Lit {
            kind: rustc_ast::token::LitKind::Float,
            symbol: sym::float_zero,
            suffix: None,
        }),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let body_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Binary(
            rustc_span::source_map::Spanned {
                node: ast::BinOpKind::Ne,
                span: call_site,
            },
            deref_self,
            zero_lit,
        ),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    build_truthy_impl_for_type(def_site, call_site, trait_path, ty_name, body_expr)
}

/// Build impl Truthy for &str { fn is_truthy(&self) -> bool { !self.is_empty() } }
fn build_truthy_impl_str_ref(
    def_site: Span,
    call_site: Span,
    trait_path: &ast::Path,
) -> Box<ast::Item> {
    // Body: !self.is_empty()
    let self_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(
            Ident::with_dummy_span(kw::SelfLower).with_span_pos(call_site)
        )),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let is_empty_call = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::is_empty, call_site)),
            receiver: self_expr,
            args: ThinVec::new(),
            span: call_site,
        })),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let body_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Unary(ast::UnOp::Not, is_empty_call),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // Build &str type
    let str_ref_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Ref(
            None,
            ast::MutTy {
                ty: Box::new(ast::Ty {
                    id: ast::DUMMY_NODE_ID,
                    kind: ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(sym::str, call_site))),
                    span: call_site,
                    tokens: None,
                }),
                mutbl: ast::Mutability::Not,
            },
        ),
        span: call_site,
        tokens: None,
    });

    build_truthy_impl_with_ty(def_site, call_site, trait_path, str_ref_ty, body_expr)
}

/// Build impl Truthy for String { fn is_truthy(&self) -> bool { !self.is_empty() } }
fn build_truthy_impl_string(
    def_site: Span,
    call_site: Span,
    trait_path: &ast::Path,
) -> Box<ast::Item> {
    // Body: !self.is_empty()
    let self_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(
            Ident::with_dummy_span(kw::SelfLower).with_span_pos(call_site)
        )),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let is_empty_call = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::is_empty, call_site)),
            receiver: self_expr,
            args: ThinVec::new(),
            span: call_site,
        })),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let body_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Unary(ast::UnOp::Not, is_empty_call),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    build_truthy_impl_for_type(def_site, call_site, trait_path, sym::String, body_expr)
}

/// Helper to build impl Truthy for Type { fn is_truthy(&self) -> bool { body } }
fn build_truthy_impl_for_type(
    def_site: Span,
    call_site: Span,
    trait_path: &ast::Path,
    ty_name: rustc_span::Symbol,
    body_expr: Box<ast::Expr>,
) -> Box<ast::Item> {
    let self_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(ty_name, call_site))),
        span: call_site,
        tokens: None,
    });

    build_truthy_impl_with_ty(def_site, call_site, trait_path, self_ty, body_expr)
}

/// Helper to build impl Truthy for a given type with a given body expression
fn build_truthy_impl_with_ty(
    def_site: Span,
    call_site: Span,
    trait_path: &ast::Path,
    self_ty: Box<ast::Ty>,
    body_expr: Box<ast::Expr>,
) -> Box<ast::Item> {
    // Build the is_truthy method with body
    // The parameter type needs to be &Self (reference to self)
    let fn_sig = ast::FnSig {
        decl: Box::new(ast::FnDecl {
            inputs: ThinVec::from([ast::Param {
                attrs: ThinVec::new(),
                ty: Box::new(ast::Ty {
                    id: ast::DUMMY_NODE_ID,
                    kind: ast::TyKind::Ref(
                        None,  // no explicit lifetime
                        ast::MutTy {
                            ty: Box::new(ast::Ty {
                                id: ast::DUMMY_NODE_ID,
                                kind: ast::TyKind::ImplicitSelf,
                                span: def_site,
                                tokens: None,
                            }),
                            mutbl: ast::Mutability::Not,
                        },
                    ),
                    span: def_site,
                    tokens: None,
                }),
                pat: Box::new(ast::Pat {
                    id: ast::DUMMY_NODE_ID,
                    kind: ast::PatKind::Ident(
                        ast::BindingMode::NONE,
                        Ident::with_dummy_span(kw::SelfLower).with_span_pos(def_site),
                        None,
                    ),
                    span: def_site,
                    tokens: None,
                }),
                id: ast::DUMMY_NODE_ID,
                span: def_site,
                is_placeholder: false,
            }]),
            output: ast::FnRetTy::Ty(Box::new(ast::Ty {
                id: ast::DUMMY_NODE_ID,
                kind: ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(sym::bool, call_site))),
                span: call_site,
                tokens: None,
            })),
        }),
        header: ast::FnHeader::default(),
        span: def_site,
    };

    let body_block = Box::new(ast::Block {
        stmts: ThinVec::from([ast::Stmt {
            id: ast::DUMMY_NODE_ID,
            kind: ast::StmtKind::Expr(body_expr),
            span: def_site,
        }]),
        id: ast::DUMMY_NODE_ID,
        rules: ast::BlockCheckMode::Default,
        span: def_site,
        tokens: None,
    });

    let fn_def = ast::Fn {
        defaultness: ast::Defaultness::Final,
        ident: Ident::new(sym::is_truthy, call_site),
        generics: ast::Generics::default(),
        sig: fn_sig,
        contract: None,
        body: Some(body_block),
        define_opaque: None,
        eii_impls: ThinVec::new(),
    };

    let impl_item = Box::new(ast::Item {
        attrs: ThinVec::new(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::AssocItemKind::Fn(Box::new(fn_def)),
        vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
        span: def_site,
        tokens: None,
    });

    let impl_def = ast::Impl {
        generics: ast::Generics::default(),
        constness: ast::Const::No,
        of_trait: Some(Box::new(ast::TraitImplHeader {
            defaultness: ast::Defaultness::Final,
            safety: ast::Safety::Default,
            polarity: ast::ImplPolarity::Positive,
            trait_ref: ast::TraitRef { path: trait_path.clone(), ref_id: ast::DUMMY_NODE_ID },
        })),
        self_ty,
        items: ThinVec::from([impl_item]),
    };

    Box::new(ast::Item {
        attrs: ThinVec::new(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::ItemKind::Impl(impl_def),
        vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
        span: def_site,
        tokens: None,
    })
}

/// Build trait method signatures for ScriptStrExt
fn build_str_ext_trait_items(span: Span) -> ThinVec<Box<ast::AssocItem>> {
    use rustc_span::Symbol;

    let mut items = ThinVec::new();

    // Helper to create a method signature that returns String
    let make_string_method = |name: &str| -> Box<ast::AssocItem> {
        let fn_sig = ast::FnSig {
            decl: Box::new(ast::FnDecl {
                inputs: ThinVec::from([ast::Param {
                    attrs: ThinVec::new(),
                    ty: Box::new(ast::Ty {
                        id: ast::DUMMY_NODE_ID,
                        kind: ast::TyKind::ImplicitSelf,
                        span,
                        tokens: None,
                    }),
                    pat: Box::new(ast::Pat {
                        id: ast::DUMMY_NODE_ID,
                        kind: ast::PatKind::Ident(
                            ast::BindingMode::NONE,
                            Ident::with_dummy_span(kw::SelfLower).with_span_pos( span),
                            None,
                        ),
                        span,
                        tokens: None,
                    }),
                    id: ast::DUMMY_NODE_ID,
                    span,
                    is_placeholder: false,
                }]),
                output: ast::FnRetTy::Ty(Box::new(ast::Ty {
                    id: ast::DUMMY_NODE_ID,
                    kind: ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(sym::String, span))),
                    span,
                    tokens: None,
                })),
            }),
            header: ast::FnHeader::default(),
            span,
        };

        let fn_def = ast::Fn {
            defaultness: ast::Defaultness::Final,
            ident: Ident::new(Symbol::intern(name), span),
            generics: ast::Generics::default(),
            sig: fn_sig,
            contract: None,
            body: None, // No body for trait method signature
            define_opaque: None,
            eii_impls: ThinVec::new(),
        };

        Box::new(ast::Item {
            attrs: ThinVec::new(),
            id: ast::DUMMY_NODE_ID,
            kind: ast::AssocItemKind::Fn(Box::new(fn_def)),
            vis: ast::Visibility { span, kind: ast::VisibilityKind::Inherited, tokens: None },
            span,
            tokens: None,
        })
    };

    // Helper to create a method signature that returns usize
    let make_usize_method = |name: &str| -> Box<ast::AssocItem> {
        let fn_sig = ast::FnSig {
            decl: Box::new(ast::FnDecl {
                inputs: ThinVec::from([ast::Param {
                    attrs: ThinVec::new(),
                    ty: Box::new(ast::Ty {
                        id: ast::DUMMY_NODE_ID,
                        kind: ast::TyKind::ImplicitSelf,
                        span,
                        tokens: None,
                    }),
                    pat: Box::new(ast::Pat {
                        id: ast::DUMMY_NODE_ID,
                        kind: ast::PatKind::Ident(
                            ast::BindingMode::NONE,
                            Ident::with_dummy_span(kw::SelfLower).with_span_pos( span),
                            None,
                        ),
                        span,
                        tokens: None,
                    }),
                    id: ast::DUMMY_NODE_ID,
                    span,
                    is_placeholder: false,
                }]),
                output: ast::FnRetTy::Ty(Box::new(ast::Ty {
                    id: ast::DUMMY_NODE_ID,
                    kind: ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(sym::usize, span))),
                    span,
                    tokens: None,
                })),
            }),
            header: ast::FnHeader::default(),
            span,
        };

        let fn_def = ast::Fn {
            defaultness: ast::Defaultness::Final,
            ident: Ident::new(Symbol::intern(name), span),
            generics: ast::Generics::default(),
            sig: fn_sig,
            contract: None,
            body: None,
            define_opaque: None,
            eii_impls: ThinVec::new(),
        };

        Box::new(ast::Item {
            attrs: ThinVec::new(),
            id: ast::DUMMY_NODE_ID,
            kind: ast::AssocItemKind::Fn(Box::new(fn_def)),
            vis: ast::Visibility { span, kind: ast::VisibilityKind::Inherited, tokens: None },
            span,
            tokens: None,
        })
    };

    items.push(make_string_method("first"));
    items.push(make_string_method("last"));
    items.push(make_string_method("reverse"));
    items.push(make_usize_method("size"));
    items.push(make_usize_method("length"));

    items
}

/// Build impl method bodies for ScriptStrExt using proper AST expressions
fn build_str_ext_impl_items(def_site: Span, call_site: Span) -> ThinVec<Box<ast::AssocItem>> {
    use rustc_span::Symbol;

    let mut items = ThinVec::new();

    // Helper to build `self` expression
    let self_expr = || -> Box<ast::Expr> {
        Box::new(ast::Expr {
            id: ast::DUMMY_NODE_ID,
            kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::with_dummy_span(kw::SelfLower).with_span_pos( call_site))),
            span: call_site,
            attrs: ThinVec::new(),
            tokens: None,
        })
    };

    // Helper to build method call: receiver.method(args)
    let method_call = |receiver: Box<ast::Expr>, method: &str, args: ThinVec<Box<ast::Expr>>| -> Box<ast::Expr> {
        Box::new(ast::Expr {
            id: ast::DUMMY_NODE_ID,
            kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
                seg: ast::PathSegment::from_ident(Ident::new(Symbol::intern(method), call_site)),
                receiver,
                args,
                span: call_site,
            })),
            span: call_site,
            attrs: ThinVec::new(),
            tokens: None,
        })
    };

    // Build first() body: self.chars().next().map(|c| &self[..c.len_utf8()]).unwrap_or("")
    // Simplified to: if self.is_empty() { "" } else { &self[..self.chars().next().unwrap().len_utf8()] }
    // Even simpler for ASCII-focused: self.get(..1).unwrap_or("")
    // Actually, let's use the match approach but build it as proper AST

    // For first(): match self.chars().next() { Some(c) => &self[..c.len_utf8()], None => "" }
    let first_body = build_first_expr(call_site);
    items.push(build_impl_method_with_expr("first", true, first_body, def_site, call_site));

    // For last(): match self.char_indices().last() { Some((i, _)) => &self[i..], None => "" }
    let last_body = build_last_expr(call_site);
    items.push(build_impl_method_with_expr("last", true, last_body, def_site, call_site));

    // For reverse(): self.chars().rev().collect()
    let reverse_body = build_reverse_expr(call_site);
    items.push(build_impl_method_with_expr("reverse", true, reverse_body, def_site, call_site));

    // For size(): self.len()
    let size_body = method_call(self_expr(), "len", ThinVec::new());
    items.push(build_impl_method_with_expr("size", false, size_body, def_site, call_site));

    // For length(): self.len()
    let length_body = method_call(self_expr(), "len", ThinVec::new());
    items.push(build_impl_method_with_expr("length", false, length_body, def_site, call_site));

    items
}

/// Build the expression for first(): self.chars().next().map(|c| c.to_string()).unwrap_or_default()
fn build_first_expr(span: Span) -> Box<ast::Expr> {
    // Build self.chars().next().map(|c| c.to_string()).unwrap_or_default()
    let self_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::with_dummy_span(kw::SelfLower).with_span_pos( span))),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // self.chars()
    let chars_call = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::chars, span)),
            receiver: self_expr,
            args: ThinVec::new(),
            span,
        })),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // .next()
    let next_call = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::next, span)),
            receiver: chars_call,
            args: ThinVec::new(),
            span,
        })),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // Build closure |c| c.to_string()
    let c_param = ast::Param {
        attrs: ThinVec::new(),
        ty: Box::new(ast::Ty {
            id: ast::DUMMY_NODE_ID,
            kind: ast::TyKind::Infer,
            span,
            tokens: None,
        }),
        pat: Box::new(ast::Pat {
            id: ast::DUMMY_NODE_ID,
            kind: ast::PatKind::Ident(ast::BindingMode::NONE, Ident::new(sym::c, span), None),
            span,
            tokens: None,
        }),
        id: ast::DUMMY_NODE_ID,
        span,
        is_placeholder: false,
    };

    // c.to_string()
    let c_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::new(sym::c, span))),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let to_string_call = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::to_string, span)),
            receiver: c_expr,
            args: ThinVec::new(),
            span,
        })),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let closure = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Closure(Box::new(ast::Closure {
            binder: ast::ClosureBinder::NotPresent,
            capture_clause: ast::CaptureBy::Ref,
            constness: ast::Const::No,
            coroutine_kind: None,
            movability: ast::Movability::Movable,
            fn_decl: Box::new(ast::FnDecl {
                inputs: ThinVec::from([c_param]),
                output: ast::FnRetTy::Default(span),
            }),
            body: to_string_call,
            fn_decl_span: span,
            fn_arg_span: span,
        })),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // .map(closure)
    let map_call = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::map, span)),
            receiver: next_call,
            args: ThinVec::from([closure]),
            span,
        })),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // .unwrap_or_default()
    Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::unwrap_or_default, span)),
            receiver: map_call,
            args: ThinVec::new(),
            span,
        })),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    })
}

/// Build the expression for last(): self.chars().last().map(|c| c.to_string()).unwrap_or_default()
fn build_last_expr(span: Span) -> Box<ast::Expr> {
    // Build self.chars().last().map(|c| c.to_string()).unwrap_or_default()
    let self_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::with_dummy_span(kw::SelfLower).with_span_pos( span))),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // self.chars()
    let chars_call = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::chars, span)),
            receiver: self_expr,
            args: ThinVec::new(),
            span,
        })),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // .last()
    let last_call = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::last, span)),
            receiver: chars_call,
            args: ThinVec::new(),
            span,
        })),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // Build closure |c| c.to_string()
    let c_param = ast::Param {
        attrs: ThinVec::new(),
        ty: Box::new(ast::Ty {
            id: ast::DUMMY_NODE_ID,
            kind: ast::TyKind::Infer,
            span,
            tokens: None,
        }),
        pat: Box::new(ast::Pat {
            id: ast::DUMMY_NODE_ID,
            kind: ast::PatKind::Ident(ast::BindingMode::NONE, Ident::new(sym::c, span), None),
            span,
            tokens: None,
        }),
        id: ast::DUMMY_NODE_ID,
        span,
        is_placeholder: false,
    };

    // c.to_string()
    let c_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::new(sym::c, span))),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let to_string_call = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::to_string, span)),
            receiver: c_expr,
            args: ThinVec::new(),
            span,
        })),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let closure = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Closure(Box::new(ast::Closure {
            binder: ast::ClosureBinder::NotPresent,
            capture_clause: ast::CaptureBy::Ref,
            constness: ast::Const::No,
            coroutine_kind: None,
            movability: ast::Movability::Movable,
            fn_decl: Box::new(ast::FnDecl {
                inputs: ThinVec::from([c_param]),
                output: ast::FnRetTy::Default(span),
            }),
            body: to_string_call,
            fn_decl_span: span,
            fn_arg_span: span,
        })),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // .map(closure)
    let map_call = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::map, span)),
            receiver: last_call,
            args: ThinVec::from([closure]),
            span,
        })),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // .unwrap_or_default()
    Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::unwrap_or_default, span)),
            receiver: map_call,
            args: ThinVec::new(),
            span,
        })),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    })
}

/// Build the expression for reverse(): self.chars().rev().collect()
fn build_reverse_expr(span: Span) -> Box<ast::Expr> {
    let self_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::with_dummy_span(kw::SelfLower).with_span_pos(span))),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // self.chars()
    let chars_call = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::chars, span)),
            receiver: self_expr,
            args: ThinVec::new(),
            span,
        })),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // .rev()
    let rev_call = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::rev, span)),
            receiver: chars_call,
            args: ThinVec::new(),
            span,
        })),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // .collect()
    Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::collect, span)),
            receiver: rev_call,
            args: ThinVec::new(),
            span,
        })),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    })
}

/// Build a single impl method with a given body expression
fn build_impl_method_with_expr(
    name: &str,
    returns_string: bool,
    body_expr: Box<ast::Expr>,
    def_site: Span,
    call_site: Span,
) -> Box<ast::AssocItem> {
    use rustc_span::Symbol;

    // Return type: String or usize
    let output = if returns_string {
        ast::FnRetTy::Ty(Box::new(ast::Ty {
            id: ast::DUMMY_NODE_ID,
            kind: ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(sym::String, call_site))),
            span: call_site,
            tokens: None,
        }))
    } else {
        ast::FnRetTy::Ty(Box::new(ast::Ty {
            id: ast::DUMMY_NODE_ID,
            kind: ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(sym::usize, call_site))),
            span: call_site,
            tokens: None,
        }))
    };

    let fn_sig = ast::FnSig {
        decl: Box::new(ast::FnDecl {
            inputs: ThinVec::from([ast::Param {
                attrs: ThinVec::new(),
                ty: Box::new(ast::Ty {
                    id: ast::DUMMY_NODE_ID,
                    kind: ast::TyKind::ImplicitSelf,
                    span: def_site,
                    tokens: None,
                }),
                pat: Box::new(ast::Pat {
                    id: ast::DUMMY_NODE_ID,
                    kind: ast::PatKind::Ident(
                        ast::BindingMode::NONE,
                        Ident::with_dummy_span(kw::SelfLower).with_span_pos( def_site),
                        None,
                    ),
                    span: def_site,
                    tokens: None,
                }),
                id: ast::DUMMY_NODE_ID,
                span: def_site,
                is_placeholder: false,
            }]),
            output,
        }),
        header: ast::FnHeader::default(),
        span: def_site,
    };

    // Build body block with the expression
    let body_block = Box::new(ast::Block {
        stmts: ThinVec::from([ast::Stmt {
            id: ast::DUMMY_NODE_ID,
            kind: ast::StmtKind::Expr(body_expr),
            span: def_site,
        }]),
        id: ast::DUMMY_NODE_ID,
        rules: ast::BlockCheckMode::Default,
        span: def_site,
        tokens: None,
    });

    let fn_def = ast::Fn {
        defaultness: ast::Defaultness::Final,
        ident: Ident::new(Symbol::intern(name), call_site),
        generics: ast::Generics::default(),
        sig: fn_sig,
        contract: None,
        body: Some(body_block),
        define_opaque: None,
        eii_impls: ThinVec::new(),
    };

    Box::new(ast::Item {
        attrs: ThinVec::new(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::AssocItemKind::Fn(Box::new(fn_def)),
        vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
        span: def_site,
        tokens: None,
    })
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

/// Build a `fn main() { <stmts> }` function with #[allow(unused_mut)] for script mode.
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

    // Suppress common warnings in script mode for convenience
    let allow_unused_mut = create_allow_attr(span, sym::unused_mut);

    // Node IDs will be assigned during macro expansion
    Box::new(ast::Item {
        attrs: vec![allow_unused_mut].into(),
        id: ast::DUMMY_NODE_ID,
        kind: main_fn,
        vis: ast::Visibility { span, kind: ast::VisibilityKind::Public, tokens: None },
        span,
        tokens: None,
    })
}
