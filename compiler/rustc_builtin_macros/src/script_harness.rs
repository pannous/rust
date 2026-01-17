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
    let type_aliases = build_type_aliases(call_site);
    let script_macros = build_script_macros(def_site, call_site);
    let string_helpers = build_string_helpers(def_site, call_site);
    // TODO: Fix slice extension trait implementation
    // let slice_helpers = build_slice_helpers(def_site, call_site);
    let truthy_helpers = build_truthy_helpers(def_site, call_site);
    let val_helpers = build_val_helpers(def_site, call_site);
    let main_fn = build_main(def_site, main_stmts);

    // Rebuild crate with type aliases + script macros + helpers + module items + main function
    krate.items = type_aliases;
    krate.items.extend(script_macros);
    krate.items.extend(string_helpers);
    // krate.items.extend(slice_helpers);
    krate.items.extend(truthy_helpers);
    krate.items.extend(val_helpers);
    krate.items.extend(module_items);
    krate.items.push(main_fn);
}

/// Build type aliases for script mode: type int = i64; type float = f64;
fn build_type_aliases(span: Span) -> ThinVec<Box<ast::Item>> {
    let mut items = ThinVec::new();

    let make_alias = |name: rustc_span::Symbol, target: rustc_span::Symbol| -> Box<ast::Item> {
        Box::new(ast::Item {
            attrs: ThinVec::new(),
            id: ast::DUMMY_NODE_ID,
            kind: ast::ItemKind::TyAlias(Box::new(ast::TyAlias {
                defaultness: ast::Defaultness::Final,
                ident: Ident::new(name, span),
                generics: ast::Generics::default(),
                after_where_clause: ast::WhereClause {
                    has_where_token: false,
                    predicates: ThinVec::new(),
                    span,
                },
                bounds: Vec::new(),
                ty: Some(build_simple_ty(span, target)),
            })),
            vis: ast::Visibility { span, kind: ast::VisibilityKind::Inherited, tokens: None },
            span,
            tokens: None,
        })
    };

    items.push(make_alias(sym::int, sym::i64));
    items.push(make_alias(sym::float, sym::f64));
    items.push(make_alias(sym::boolean, sym::bool));
    items.push(make_alias(sym::rune, sym::char));
    items.push(make_alias(sym::byte, sym::u8));
    items.push(make_alias(sym::string, sym::String));

    items
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

/// Build helper trait and impl for slice methods: filter()
/// Generates:
/// ```ignore
/// trait ScriptSliceExt<T: Clone> {
///     fn filter<F: Fn(&T) -> bool>(&self, f: F) -> Vec<T>;
/// }
/// impl<T: Clone> ScriptSliceExt<T> for [T] {
///     fn filter<F: Fn(&T) -> bool>(&self, f: F) -> Vec<T> {
///         self.iter().filter(|x| f(x)).cloned().collect()
///     }
/// }
/// ```
#[allow(dead_code)]
fn build_slice_helpers(def_site: Span, call_site: Span) -> ThinVec<Box<ast::Item>> {
    let mut items = ThinVec::new();

    // Create #[allow(dead_code)] attribute
    let allow_dead_code = create_allow_attr(def_site, sym::dead_code);

    // Symbol for the trait name
    let trait_name = sym::ScriptSliceExt;

    // Create generic parameter T with Clone bound
    let t_ident = Ident::new(sym::T, call_site);
    let clone_bound = ast::GenericBound::Trait(
        ast::PolyTraitRef {
            bound_generic_params: ThinVec::new(),
            modifiers: ast::TraitBoundModifiers::NONE,
            trait_ref: ast::TraitRef {
                path: ast::Path::from_ident(Ident::new(sym::Clone, call_site)),
                ref_id: ast::DUMMY_NODE_ID,
            },
            span: call_site,
            parens: ast::Parens::No,
        },
    );

    let t_param = ast::GenericParam {
        id: ast::DUMMY_NODE_ID,
        ident: t_ident,
        attrs: ThinVec::new(),
        bounds: vec![clone_bound.clone()],
        is_placeholder: false,
        kind: ast::GenericParamKind::Type { default: None },
        colon_span: None,
    };

    let trait_generics = ast::Generics {
        params: ThinVec::from([t_param.clone()]),
        where_clause: ast::WhereClause {
            has_where_token: false,
            predicates: ThinVec::new(),
            span: call_site,
        },
        span: call_site,
    };

    // Build the filter method signature for the trait
    // fn filter<F: Fn(&T) -> bool>(&self, f: F) -> Vec<T>;
    let filter_trait_item = build_slice_filter_trait_item(call_site, t_ident);

    // Build the trait definition
    let trait_def = ast::Trait {
        constness: ast::Const::No,
        safety: ast::Safety::Default,
        is_auto: ast::IsAuto::No,
        ident: Ident::new(trait_name, call_site),
        generics: trait_generics,
        bounds: Vec::new(),
        items: ThinVec::from([filter_trait_item]),
    };

    items.push(Box::new(ast::Item {
        attrs: vec![allow_dead_code.clone()].into(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::ItemKind::Trait(Box::new(trait_def)),
        vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
        span: def_site,
        tokens: None,
    }));

    // Build the impl block for [T]
    // impl<T: Clone> ScriptSliceExt<T> for [T] { ... }
    let impl_item = build_slice_ext_impl(def_site, call_site, trait_name, t_ident, clone_bound);
    items.push(impl_item);

    items
}

/// Build the filter method signature for ScriptSliceExt trait
#[allow(dead_code)]
fn build_slice_filter_trait_item(span: Span, t_ident: Ident) -> Box<ast::AssocItem> {
    // Create generic parameter F with Fn(&T) -> bool bound
    let f_ident = Ident::new(sym::F, span);

    // Build &T type for the closure argument
    let t_ref_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Ref(
            None,
            ast::MutTy {
                ty: Box::new(ast::Ty {
                    id: ast::DUMMY_NODE_ID,
                    kind: ast::TyKind::Path(None, ast::Path::from_ident(t_ident)),
                    span,
                    tokens: None,
                }),
                mutbl: ast::Mutability::Not,
            },
        ),
        span,
        tokens: None,
    });

    // Build bool type
    let bool_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(sym::bool, span))),
        span,
        tokens: None,
    });

    // Build Fn(&T) -> bool bound for F
    // This is a trait bound with parenthesized args
    let fn_bound = ast::GenericBound::Trait(
        ast::PolyTraitRef {
            bound_generic_params: ThinVec::new(),
            modifiers: ast::TraitBoundModifiers::NONE,
            trait_ref: ast::TraitRef {
                path: ast::Path {
                    span,
                    segments: ThinVec::from([ast::PathSegment {
                        ident: Ident::new(sym::Fn, span),
                        id: ast::DUMMY_NODE_ID,
                        args: Some(Box::new(ast::GenericArgs::Parenthesized(ast::ParenthesizedArgs {
                            span,
                            inputs: ThinVec::from([t_ref_ty.clone()]),
                            inputs_span: span,
                            output: ast::FnRetTy::Ty(bool_ty.clone()),
                        }))),
                    }]),
                    tokens: None,
                },
                ref_id: ast::DUMMY_NODE_ID,
            },
            span,
            parens: ast::Parens::No,
        },
    );

    let f_param = ast::GenericParam {
        id: ast::DUMMY_NODE_ID,
        ident: f_ident,
        attrs: ThinVec::new(),
        bounds: vec![fn_bound],
        is_placeholder: false,
        kind: ast::GenericParamKind::Type { default: None },
        colon_span: None,
    };

    let method_generics = ast::Generics {
        params: ThinVec::from([f_param]),
        where_clause: ast::WhereClause {
            has_where_token: false,
            predicates: ThinVec::new(),
            span,
        },
        span,
    };

    // Build Vec<T> return type
    let t_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(None, ast::Path::from_ident(t_ident)),
        span,
        tokens: None,
    });

    let vec_t_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(
            None,
            ast::Path {
                span,
                segments: ThinVec::from([ast::PathSegment {
                    ident: Ident::new(sym::Vec, span),
                    id: ast::DUMMY_NODE_ID,
                    args: Some(Box::new(ast::GenericArgs::AngleBracketed(ast::AngleBracketedArgs {
                        span,
                        args: ThinVec::from([ast::AngleBracketedArg::Arg(ast::GenericArg::Type(t_ty))]),
                    }))),
                }]),
                tokens: None,
            },
        ),
        span,
        tokens: None,
    });

    // Build &self parameter
    let self_param = ast::Param {
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
                Ident::new(kw::SelfLower, span),
                None,
            ),
            span,
            tokens: None,
        }),
        id: ast::DUMMY_NODE_ID,
        span,
        is_placeholder: false,
    };

    // Build f: F parameter
    let f_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(None, ast::Path::from_ident(f_ident)),
        span,
        tokens: None,
    });

    let f_param_decl = ast::Param {
        attrs: ThinVec::new(),
        ty: f_ty,
        pat: Box::new(ast::Pat {
            id: ast::DUMMY_NODE_ID,
            kind: ast::PatKind::Ident(
                ast::BindingMode::NONE,
                Ident::new(sym::f, span),
                None,
            ),
            span,
            tokens: None,
        }),
        id: ast::DUMMY_NODE_ID,
        span,
        is_placeholder: false,
    };

    let fn_sig = ast::FnSig {
        header: ast::FnHeader::default(),
        decl: Box::new(ast::FnDecl {
            inputs: ThinVec::from([self_param, f_param_decl]),
            output: ast::FnRetTy::Ty(vec_t_ty),
        }),
        span,
    };

    Box::new(ast::AssocItem {
        attrs: ThinVec::new(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::AssocItemKind::Fn(Box::new(ast::Fn {
            defaultness: ast::Defaultness::Final,
            ident: Ident::new(sym::filter, span),
            generics: method_generics,
            sig: fn_sig,
            contract: None,
            body: None, // No body for trait method
            define_opaque: None,
            eii_impls: ThinVec::new(),
        })),
        vis: ast::Visibility { span, kind: ast::VisibilityKind::Inherited, tokens: None },
        span,
        tokens: None,
    })
}

/// Build impl<T: Clone> ScriptSliceExt<T> for [T] { ... }
#[allow(dead_code)]
fn build_slice_ext_impl(
    def_site: Span,
    call_site: Span,
    trait_name: rustc_span::Symbol,
    t_ident: Ident,
    clone_bound: ast::GenericBound,
) -> Box<ast::Item> {
    // Create generic parameter T with Clone bound for the impl
    let t_param = ast::GenericParam {
        id: ast::DUMMY_NODE_ID,
        ident: t_ident,
        attrs: ThinVec::new(),
        bounds: vec![clone_bound],
        is_placeholder: false,
        kind: ast::GenericParamKind::Type { default: None },
        colon_span: None,
    };

    let impl_generics = ast::Generics {
        params: ThinVec::from([t_param]),
        where_clause: ast::WhereClause {
            has_where_token: false,
            predicates: ThinVec::new(),
            span: call_site,
        },
        span: call_site,
    };

    // Build T type
    let t_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(None, ast::Path::from_ident(t_ident)),
        span: call_site,
        tokens: None,
    });

    // Build [T] type (slice)
    let slice_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Slice(t_ty.clone()),
        span: call_site,
        tokens: None,
    });

    // Build ScriptSliceExt<T> trait reference
    let trait_path = ast::Path {
        span: call_site,
        segments: ThinVec::from([ast::PathSegment {
            ident: Ident::new(trait_name, call_site),
            id: ast::DUMMY_NODE_ID,
            args: Some(Box::new(ast::GenericArgs::AngleBracketed(ast::AngleBracketedArgs {
                span: call_site,
                args: ThinVec::from([ast::AngleBracketedArg::Arg(ast::GenericArg::Type(t_ty.clone()))]),
            }))),
        }]),
        tokens: None,
    };

    // Build the filter method implementation
    let filter_impl_item = build_slice_filter_impl_item(call_site, t_ident);

    let impl_def = ast::Impl {
        generics: impl_generics,
        constness: ast::Const::No,
        of_trait: Some(Box::new(ast::TraitImplHeader {
            defaultness: ast::Defaultness::Final,
            safety: ast::Safety::Default,
            polarity: ast::ImplPolarity::Positive,
            trait_ref: ast::TraitRef { path: trait_path, ref_id: ast::DUMMY_NODE_ID },
        })),
        self_ty: slice_ty,
        items: ThinVec::from([filter_impl_item]),
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

/// Build the filter method implementation: self.iter().filter(|x| f(x)).cloned().collect()
#[allow(dead_code)]
fn build_slice_filter_impl_item(span: Span, t_ident: Ident) -> Box<ast::AssocItem> {
    // Create generic parameter F with Fn(&T) -> bool bound (same as trait)
    let f_ident = Ident::new(sym::F, span);

    // Build &T type
    let t_ref_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Ref(
            None,
            ast::MutTy {
                ty: Box::new(ast::Ty {
                    id: ast::DUMMY_NODE_ID,
                    kind: ast::TyKind::Path(None, ast::Path::from_ident(t_ident)),
                    span,
                    tokens: None,
                }),
                mutbl: ast::Mutability::Not,
            },
        ),
        span,
        tokens: None,
    });

    // Build bool type
    let bool_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(sym::bool, span))),
        span,
        tokens: None,
    });

    // Build Fn(&T) -> bool bound
    let fn_bound = ast::GenericBound::Trait(
        ast::PolyTraitRef {
            bound_generic_params: ThinVec::new(),
            modifiers: ast::TraitBoundModifiers::NONE,
            trait_ref: ast::TraitRef {
                path: ast::Path {
                    span,
                    segments: ThinVec::from([ast::PathSegment {
                        ident: Ident::new(sym::Fn, span),
                        id: ast::DUMMY_NODE_ID,
                        args: Some(Box::new(ast::GenericArgs::Parenthesized(ast::ParenthesizedArgs {
                            span,
                            inputs: ThinVec::from([t_ref_ty.clone()]),
                            inputs_span: span,
                            output: ast::FnRetTy::Ty(bool_ty),
                        }))),
                    }]),
                    tokens: None,
                },
                ref_id: ast::DUMMY_NODE_ID,
            },
            span,
            parens: ast::Parens::No,
        },
    );

    let f_param = ast::GenericParam {
        id: ast::DUMMY_NODE_ID,
        ident: f_ident,
        attrs: ThinVec::new(),
        bounds: vec![fn_bound],
        is_placeholder: false,
        kind: ast::GenericParamKind::Type { default: None },
        colon_span: None,
    };

    let method_generics = ast::Generics {
        params: ThinVec::from([f_param]),
        where_clause: ast::WhereClause {
            has_where_token: false,
            predicates: ThinVec::new(),
            span,
        },
        span,
    };

    // Build Vec<T> return type
    let t_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(None, ast::Path::from_ident(t_ident)),
        span,
        tokens: None,
    });

    let vec_t_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(
            None,
            ast::Path {
                span,
                segments: ThinVec::from([ast::PathSegment {
                    ident: Ident::new(sym::Vec, span),
                    id: ast::DUMMY_NODE_ID,
                    args: Some(Box::new(ast::GenericArgs::AngleBracketed(ast::AngleBracketedArgs {
                        span,
                        args: ThinVec::from([ast::AngleBracketedArg::Arg(ast::GenericArg::Type(t_ty))]),
                    }))),
                }]),
                tokens: None,
            },
        ),
        span,
        tokens: None,
    });

    // Build &self parameter
    let self_param = ast::Param {
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
                Ident::new(kw::SelfLower, span),
                None,
            ),
            span,
            tokens: None,
        }),
        id: ast::DUMMY_NODE_ID,
        span,
        is_placeholder: false,
    };

    // Build f: F parameter
    let f_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(None, ast::Path::from_ident(f_ident)),
        span,
        tokens: None,
    });

    let f_param_decl = ast::Param {
        attrs: ThinVec::new(),
        ty: f_ty,
        pat: Box::new(ast::Pat {
            id: ast::DUMMY_NODE_ID,
            kind: ast::PatKind::Ident(
                ast::BindingMode::NONE,
                Ident::new(sym::f, span),
                None,
            ),
            span,
            tokens: None,
        }),
        id: ast::DUMMY_NODE_ID,
        span,
        is_placeholder: false,
    };

    let fn_sig = ast::FnSig {
        header: ast::FnHeader::default(),
        decl: Box::new(ast::FnDecl {
            inputs: ThinVec::from([self_param, f_param_decl]),
            output: ast::FnRetTy::Ty(vec_t_ty),
        }),
        span,
    };

    // Build the method body: self.iter().filter(|x| f(x)).cloned().collect()
    let body = build_filter_body(span);

    Box::new(ast::AssocItem {
        attrs: ThinVec::new(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::AssocItemKind::Fn(Box::new(ast::Fn {
            defaultness: ast::Defaultness::Final,
            ident: Ident::new(sym::filter, span),
            generics: method_generics,
            sig: fn_sig,
            contract: None,
            body: Some(body),
            define_opaque: None,
            eii_impls: ThinVec::new(),
        })),
        vis: ast::Visibility { span, kind: ast::VisibilityKind::Inherited, tokens: None },
        span,
        tokens: None,
    })
}

/// simple list.filter(criterion) =>
/// Build the filter method body: self.iter().filter(|x| f(x)).cloned().collect()
#[allow(dead_code)]
fn build_filter_body(span: Span) -> Box<ast::Block> {
    // Build: self
    let self_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::new(kw::SelfLower, span))),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // Build: self.iter()
    let iter_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::iter, span)),
            receiver: self_expr,
            args: ThinVec::new(),
            span,
        })),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // Build closure: |x| f(x)
    // First build f(x)
    let f_path = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::new(sym::f, span))),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let x_path = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::new(sym::x, span))),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let f_call = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Call(f_path, ThinVec::from([x_path])),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // Build closure parameter |x|
    let x_param = ast::Param {
        attrs: ThinVec::new(),
        ty: Box::new(ast::Ty {
            id: ast::DUMMY_NODE_ID,
            kind: ast::TyKind::Infer,
            span,
            tokens: None,
        }),
        pat: Box::new(ast::Pat {
            id: ast::DUMMY_NODE_ID,
            kind: ast::PatKind::Ident(
                ast::BindingMode::NONE,
                Ident::new(sym::x, span),
                None,
            ),
            span,
            tokens: None,
        }),
        id: ast::DUMMY_NODE_ID,
        span,
        is_placeholder: false,
    };

    let closure = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Closure(Box::new(ast::Closure {
            binder: ast::ClosureBinder::NotPresent,
            capture_clause: ast::CaptureBy::Ref,
            constness: ast::Const::No,
            coroutine_kind: None,
            movability: ast::Movability::Movable,
            fn_decl: Box::new(ast::FnDecl {
                inputs: ThinVec::from([x_param]),
                output: ast::FnRetTy::Default(span),
            }),
            body: f_call,
            fn_decl_span: span,
            fn_arg_span: span,
        })),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // Build: .filter(|x| f(x))
    let filter_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::filter, span)),
            receiver: iter_expr,
            args: ThinVec::from([closure]),
            span,
        })),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // Build: .cloned()
    let cloned_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::cloned, span)),
            receiver: filter_expr,
            args: ThinVec::new(),
            span,
        })),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // Build: .collect()
    let collect_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::collect, span)),
            receiver: cloned_expr,
            args: ThinVec::new(),
            span,
        })),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // Wrap in a block
    Box::new(ast::Block {
        stmts: ThinVec::from([ast::Stmt {
            id: ast::DUMMY_NODE_ID,
            kind: ast::StmtKind::Expr(collect_expr),
            span,
        }]),
        id: ast::DUMMY_NODE_ID,
        rules: ast::BlockCheckMode::Default,
        span,
        tokens: None,
    })
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

    // impl<T> Truthy for Vec<T>
    items.push(build_truthy_impl_vec(def_site, call_site, &trait_path));

    // impl<T> Truthy for Option<T>
    items.push(build_truthy_impl_option(def_site, call_site, &trait_path));

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

/// Build impl<T> Truthy for Vec<T> { fn is_truthy(&self) -> bool { !self.is_empty() } }
fn build_truthy_impl_vec(
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

    build_truthy_impl_generic(def_site, call_site, trait_path, sym::Vec, body_expr)
}

/// Build impl<T> Truthy for Option<T> { fn is_truthy(&self) -> bool { self.is_some() } }
fn build_truthy_impl_option(
    def_site: Span,
    call_site: Span,
    trait_path: &ast::Path,
) -> Box<ast::Item> {
    // Body: self.is_some()
    let self_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(
            Ident::with_dummy_span(kw::SelfLower).with_span_pos(call_site)
        )),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let body_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::is_some, call_site)),
            receiver: self_expr,
            args: ThinVec::new(),
            span: call_site,
        })),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    build_truthy_impl_generic(def_site, call_site, trait_path, sym::Option, body_expr)
}

/// Helper to build impl<T> Truthy for Type<T> { fn is_truthy(&self) -> bool { body } }
fn build_truthy_impl_generic(
    def_site: Span,
    call_site: Span,
    trait_path: &ast::Path,
    ty_name: rustc_span::Symbol,
    body_expr: Box<ast::Expr>,
) -> Box<ast::Item> {
    // Create generic parameter T
    let t_ident = Ident::new(sym::T, call_site);
    let generic_param = ast::GenericParam {
        id: ast::DUMMY_NODE_ID,
        ident: t_ident,
        attrs: ThinVec::new(),
        bounds: Vec::new(),
        is_placeholder: false,
        kind: ast::GenericParamKind::Type { default: None },
        colon_span: None,
    };

    let generics = ast::Generics {
        params: ThinVec::from([generic_param]),
        where_clause: ast::WhereClause {
            has_where_token: false,
            predicates: ThinVec::new(),
            span: call_site,
        },
        span: call_site,
    };

    // Build Vec<T> or Option<T> type
    let t_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(None, ast::Path::from_ident(t_ident)),
        span: call_site,
        tokens: None,
    });

    let generic_args = ast::GenericArgs::AngleBracketed(ast::AngleBracketedArgs {
        span: call_site,
        args: ThinVec::from([ast::AngleBracketedArg::Arg(ast::GenericArg::Type(t_ty))]),
    });

    let self_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(
            None,
            ast::Path {
                span: call_site,
                segments: ThinVec::from([ast::PathSegment {
                    ident: Ident::new(ty_name, call_site),
                    id: ast::DUMMY_NODE_ID,
                    args: Some(Box::new(generic_args)),
                }]),
                tokens: None,
            },
        ),
        span: call_site,
        tokens: None,
    });

    build_truthy_impl_with_ty_and_generics(def_site, call_site, trait_path, self_ty, body_expr, generics)
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

/// Helper to build impl<T> Truthy for Type<T> with generics
fn build_truthy_impl_with_ty_and_generics(
    def_site: Span,
    call_site: Span,
    trait_path: &ast::Path,
    self_ty: Box<ast::Ty>,
    body_expr: Box<ast::Expr>,
    generics: ast::Generics,
) -> Box<ast::Item> {
    // Build the is_truthy method with body
    let fn_sig = ast::FnSig {
        decl: Box::new(ast::FnDecl {
            inputs: ThinVec::from([ast::Param {
                attrs: ThinVec::new(),
                ty: Box::new(ast::Ty {
                    id: ast::DUMMY_NODE_ID,
                    kind: ast::TyKind::Ref(
                        None,
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
        generics,  // Use the provided generics
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

/// Build Val enum and related implementations for mixed-type lists.
/// Generates:
/// ```ignore
/// #[derive(Clone, Debug)]
/// enum Val {
///     Str(String),
///     Int(i64),
///     Float(f64),
///     Bool(bool),
///     List(Vec<Val>),
///     Nil,
/// }
/// impl std::fmt::Display for Val { ... }
/// impl From<&str> for Val { ... }
/// impl From<String> for Val { ... }
/// impl From<i64> for Val { ... }
/// impl From<i32> for Val { ... }
/// impl From<f64> for Val { ... }
/// impl From<f32> for Val { ... }
/// impl From<bool> for Val { ... }
/// impl Truthy for Val { ... }
/// ```
fn build_val_helpers(def_site: Span, call_site: Span) -> ThinVec<Box<ast::Item>> {
    let mut items = ThinVec::new();

    // Create #[allow(dead_code)] attribute
    let allow_dead_code = create_allow_attr(def_site, sym::dead_code);

    // Build the Val enum definition
    items.push(build_val_enum(def_site, call_site, allow_dead_code.clone()));

    // Build Display impl
    items.push(build_val_display_impl(def_site, call_site));

    // Build From impls
    items.push(build_val_from_str_ref(def_site, call_site));
    items.push(build_val_from_string(def_site, call_site));
    items.push(build_val_from_i64(def_site, call_site));
    items.push(build_val_from_i32(def_site, call_site));
    items.push(build_val_from_f64(def_site, call_site));
    items.push(build_val_from_f32(def_site, call_site));
    items.push(build_val_from_bool(def_site, call_site));

    // Build Truthy impl
    let truthy_path = ast::Path::from_ident(Ident::new(sym::Truthy, call_site));
    items.push(build_val_truthy_impl(def_site, call_site, &truthy_path));

    items
}

/// Build the Val enum definition
fn build_val_enum(def_site: Span, call_site: Span, allow_dead_code: ast::Attribute) -> Box<ast::Item> {
    // Helper to create a variant with a single tuple field
    let tuple_variant = |name: rustc_span::Symbol, field_ty: ast::TyKind| -> ast::Variant {
        ast::Variant {
            attrs: ThinVec::new(),
            id: ast::DUMMY_NODE_ID,
            span: def_site,
            vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
            ident: Ident::new(name, call_site),
            data: ast::VariantData::Tuple(
                ThinVec::from([ast::FieldDef {
                    attrs: ThinVec::new(),
                    id: ast::DUMMY_NODE_ID,
                    span: def_site,
                    vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
                    ident: None,
                    ty: Box::new(ast::Ty {
                        id: ast::DUMMY_NODE_ID,
                        kind: field_ty,
                        span: call_site,
                        tokens: None,
                    }),
                    is_placeholder: false,
                    safety: ast::Safety::Default,
                    default: None,
                }]),
                ast::DUMMY_NODE_ID,
            ),
            disr_expr: None,
            is_placeholder: false,
        }
    };

    // Helper to create a unit variant
    let unit_variant = |name: rustc_span::Symbol| -> ast::Variant {
        ast::Variant {
            attrs: ThinVec::new(),
            id: ast::DUMMY_NODE_ID,
            span: def_site,
            vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
            ident: Ident::new(name, call_site),
            data: ast::VariantData::Unit(ast::DUMMY_NODE_ID),
            disr_expr: None,
            is_placeholder: false,
        }
    };

    // Build Vec<Val> type for List variant
    let val_ty = ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(sym::Val, call_site)));
    let vec_val_ty = ast::TyKind::Path(
        None,
        ast::Path {
            span: call_site,
            segments: ThinVec::from([ast::PathSegment {
                ident: Ident::new(sym::Vec, call_site),
                id: ast::DUMMY_NODE_ID,
                args: Some(Box::new(ast::GenericArgs::AngleBracketed(ast::AngleBracketedArgs {
                    span: call_site,
                    args: ThinVec::from([ast::AngleBracketedArg::Arg(ast::GenericArg::Type(Box::new(ast::Ty {
                        id: ast::DUMMY_NODE_ID,
                        kind: val_ty,
                        span: call_site,
                        tokens: None,
                    })))]),
                }))),
            }]),
            tokens: None,
        },
    );

    let variants = ThinVec::from([
        tuple_variant(sym::Str, ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(sym::String, call_site)))),
        tuple_variant(sym::Int, ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(sym::i64, call_site)))),
        tuple_variant(sym::Float, ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(sym::f64, call_site)))),
        tuple_variant(sym::Bool, ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(sym::bool, call_site)))),
        tuple_variant(sym::List, vec_val_ty),
        unit_variant(sym::Nil),
    ]);

    let enum_def = ast::EnumDef { variants };

    // Create #[derive(Clone, Debug)] attribute
    let derive_attr = create_derive_attr(def_site, &[sym::Clone, sym::Debug]);

    Box::new(ast::Item {
        attrs: ThinVec::from([allow_dead_code, derive_attr]),
        id: ast::DUMMY_NODE_ID,
        kind: ast::ItemKind::Enum(Ident::new(sym::Val, call_site), ast::Generics::default(), enum_def),
        vis: ast::Visibility { span: def_site, kind: ast::VisibilityKind::Inherited, tokens: None },
        span: def_site,
        tokens: None,
    })
}

/// Create #[derive(Trait1, Trait2, ...)] attribute
fn create_derive_attr(span: Span, traits: &[rustc_span::Symbol]) -> ast::Attribute {
    use rustc_ast::{AttrArgs, AttrItemKind, AttrKind, AttrStyle, NormalAttr, Path, PathSegment, Safety};
    use rustc_ast::token::{IdentIsRaw, TokenKind};
    use rustc_ast::tokenstream::{TokenStream, TokenTree};

    let path = Path {
        span,
        segments: ThinVec::from([PathSegment::from_ident(Ident::new(sym::derive, span))]),
        tokens: None,
    };

    // Build token stream for traits: Clone, Debug
    let mut tokens = Vec::new();
    for (i, &trait_sym) in traits.iter().enumerate() {
        if i > 0 {
            tokens.push(TokenTree::token_alone(TokenKind::Comma, span));
        }
        tokens.push(TokenTree::token_alone(TokenKind::Ident(trait_sym, IdentIsRaw::No), span));
    }

    let args = AttrArgs::Delimited(ast::DelimArgs {
        dspan: rustc_ast::tokenstream::DelimSpan::from_single(span),
        delim: rustc_ast::token::Delimiter::Parenthesis,
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

/// Build impl std::fmt::Display for Val
fn build_val_display_impl(def_site: Span, call_site: Span) -> Box<ast::Item> {
    // Build the fmt method body using a match expression
    // match self {
    //     Val::Str(s) => write!(f, "{}", s),
    //     Val::Int(n) => write!(f, "{}", n),
    //     Val::Float(n) => write!(f, "{}", n),
    //     Val::Bool(b) => write!(f, "{}", b),
    //     Val::List(v) => write!(f, "{:?}", v),
    //     Val::Nil => write!(f, "nil"),
    // }

    // This is complex to build as AST, so we'll create a simplified version
    // that uses format! with debug formatting
    // For now, just use Debug formatting: write!(f, "{:?}", self)

    let self_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(
            Ident::with_dummy_span(kw::SelfLower).with_span_pos(call_site)
        )),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let f_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::new(sym::f, call_site))),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // Build match expression for proper display
    let match_expr = build_val_display_match(call_site, self_expr, f_expr);

    // Build fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    let fn_sig = build_display_fn_sig(def_site, call_site);

    let body_block = Box::new(ast::Block {
        stmts: ThinVec::from([ast::Stmt {
            id: ast::DUMMY_NODE_ID,
            kind: ast::StmtKind::Expr(match_expr),
            span: def_site,
        }]),
        id: ast::DUMMY_NODE_ID,
        rules: ast::BlockCheckMode::Default,
        span: def_site,
        tokens: None,
    });

    let fn_def = ast::Fn {
        defaultness: ast::Defaultness::Final,
        ident: Ident::new(sym::fmt, call_site),
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

    // Build std::fmt::Display trait path
    let display_path = ast::Path {
        span: call_site,
        segments: ThinVec::from([
            ast::PathSegment::from_ident(Ident::new(sym::std, call_site)),
            ast::PathSegment::from_ident(Ident::new(sym::fmt, call_site)),
            ast::PathSegment::from_ident(Ident::new(sym::Display, call_site)),
        ]),
        tokens: None,
    };

    let val_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(sym::Val, call_site))),
        span: call_site,
        tokens: None,
    });

    let impl_def = ast::Impl {
        generics: ast::Generics::default(),
        constness: ast::Const::No,
        of_trait: Some(Box::new(ast::TraitImplHeader {
            defaultness: ast::Defaultness::Final,
            safety: ast::Safety::Default,
            polarity: ast::ImplPolarity::Positive,
            trait_ref: ast::TraitRef { path: display_path, ref_id: ast::DUMMY_NODE_ID },
        })),
        self_ty: val_ty,
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

/// Build the Display fn signature: fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
fn build_display_fn_sig(def_site: Span, call_site: Span) -> ast::FnSig {
    // Build &mut std::fmt::Formatter<'_> type
    let formatter_path = ast::Path {
        span: call_site,
        segments: ThinVec::from([
            ast::PathSegment::from_ident(Ident::new(sym::std, call_site)),
            ast::PathSegment::from_ident(Ident::new(sym::fmt, call_site)),
            ast::PathSegment {
                ident: Ident::new(sym::Formatter, call_site),
                id: ast::DUMMY_NODE_ID,
                args: Some(Box::new(ast::GenericArgs::AngleBracketed(ast::AngleBracketedArgs {
                    span: call_site,
                    args: ThinVec::from([ast::AngleBracketedArg::Arg(ast::GenericArg::Lifetime(ast::Lifetime {
                        id: ast::DUMMY_NODE_ID,
                        ident: Ident::new(kw::UnderscoreLifetime, call_site),
                    }))]),
                }))),
            },
        ]),
        tokens: None,
    };

    let formatter_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Ref(
            None,
            ast::MutTy {
                ty: Box::new(ast::Ty {
                    id: ast::DUMMY_NODE_ID,
                    kind: ast::TyKind::Path(None, formatter_path),
                    span: call_site,
                    tokens: None,
                }),
                mutbl: ast::Mutability::Mut,
            },
        ),
        span: call_site,
        tokens: None,
    });

    // Build std::fmt::Result type
    let result_path = ast::Path {
        span: call_site,
        segments: ThinVec::from([
            ast::PathSegment::from_ident(Ident::new(sym::std, call_site)),
            ast::PathSegment::from_ident(Ident::new(sym::fmt, call_site)),
            ast::PathSegment::from_ident(Ident::new(sym::Result, call_site)),
        ]),
        tokens: None,
    };

    let result_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(None, result_path),
        span: call_site,
        tokens: None,
    });

    ast::FnSig {
        decl: Box::new(ast::FnDecl {
            inputs: ThinVec::from([
                // &self
                ast::Param {
                    attrs: ThinVec::new(),
                    ty: Box::new(ast::Ty {
                        id: ast::DUMMY_NODE_ID,
                        kind: ast::TyKind::Ref(
                            None,
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
                },
                // f: &mut Formatter<'_>
                ast::Param {
                    attrs: ThinVec::new(),
                    ty: formatter_ty,
                    pat: Box::new(ast::Pat {
                        id: ast::DUMMY_NODE_ID,
                        kind: ast::PatKind::Ident(
                            ast::BindingMode::NONE,
                            Ident::new(sym::f, call_site),
                            None,
                        ),
                        span: call_site,
                        tokens: None,
                    }),
                    id: ast::DUMMY_NODE_ID,
                    span: call_site,
                    is_placeholder: false,
                },
            ]),
            output: ast::FnRetTy::Ty(result_ty),
        }),
        header: ast::FnHeader::default(),
        span: def_site,
    }
}

/// Build match expression for Display::fmt
fn build_val_display_match(
    span: Span,
    self_expr: Box<ast::Expr>,
    f_expr: Box<ast::Expr>,
) -> Box<ast::Expr> {
    // Build match arms for each Val variant
    let arms = ThinVec::from([
        build_val_match_arm(span, sym::Str, sym::s, &f_expr, false),
        build_val_match_arm(span, sym::Int, sym::__n, &f_expr, false),
        build_val_match_arm(span, sym::Float, sym::__n, &f_expr, false),
        build_val_match_arm(span, sym::Bool, sym::__b, &f_expr, false),
        build_val_match_arm(span, sym::List, sym::__v, &f_expr, true), // Use debug format
        build_val_nil_match_arm(span, &f_expr),
    ]);

    Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Match(self_expr, arms, ast::MatchKind::Prefix),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    })
}

/// Build a match arm: Val::Variant(binding) => write!(f, "{}", binding)
fn build_val_match_arm(
    span: Span,
    variant: rustc_span::Symbol,
    binding: rustc_span::Symbol,
    f_expr: &Box<ast::Expr>,
    use_debug: bool,
) -> ast::Arm {
    // Pattern: Val::Variant(binding)
    let pat_path = ast::Path {
        span,
        segments: ThinVec::from([
            ast::PathSegment::from_ident(Ident::new(sym::Val, span)),
            ast::PathSegment::from_ident(Ident::new(variant, span)),
        ]),
        tokens: None,
    };

    let binding_pat = ast::Pat {
        id: ast::DUMMY_NODE_ID,
        kind: ast::PatKind::Ident(
            ast::BindingMode::NONE,
            Ident::new(binding, span),
            None,
        ),
        span,
        tokens: None,
    };

    let pat = Box::new(ast::Pat {
        id: ast::DUMMY_NODE_ID,
        kind: ast::PatKind::TupleStruct(None, pat_path, ThinVec::from([binding_pat])),
        span,
        tokens: None,
    });

    // Body: write!(f, "{}" or "{:?}", binding)
    let binding_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::new(binding, span))),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let body = build_write_call(span, f_expr.clone(), binding_expr, use_debug);

    ast::Arm {
        attrs: ThinVec::new(),
        pat,
        guard: None,
        body: Some(body),
        span,
        id: ast::DUMMY_NODE_ID,
        is_placeholder: false,
    }
}

/// Build match arm for Nil: Val::Nil => write!(f, "nil")
fn build_val_nil_match_arm(span: Span, f_expr: &Box<ast::Expr>) -> ast::Arm {
    // Pattern: Val::Nil
    let pat_path = ast::Path {
        span,
        segments: ThinVec::from([
            ast::PathSegment::from_ident(Ident::new(sym::Val, span)),
            ast::PathSegment::from_ident(Ident::new(sym::Nil, span)),
        ]),
        tokens: None,
    };

    let pat = Box::new(ast::Pat {
        id: ast::DUMMY_NODE_ID,
        kind: ast::PatKind::Path(None, pat_path),
        span,
        tokens: None,
    });

    // Body: write!(f, "nil")
    let nil_str = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Lit(rustc_ast::token::Lit {
            kind: rustc_ast::token::LitKind::Str,
            symbol: sym::nil,
            suffix: None,
        }),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let body = build_write_str_call(span, f_expr.clone(), nil_str);

    ast::Arm {
        attrs: ThinVec::new(),
        pat,
        guard: None,
        body: Some(body),
        span,
        id: ast::DUMMY_NODE_ID,
        is_placeholder: false,
    }
}

/// Build write!(f, "{}", expr) call
fn build_write_call(span: Span, _f_expr: Box<ast::Expr>, val_expr: Box<ast::Expr>, use_debug: bool) -> Box<ast::Expr> {
    use rustc_ast::token::{self, Delimiter, Lit, LitKind, TokenKind};
    use rustc_ast::tokenstream::{DelimSpan, TokenStream, TokenTree};

    let fmt_sym = if use_debug { sym::empty_braces_debug } else { sym::empty_braces };

    // Build token stream for write!(f, "{}", val)
    let tokens = vec![
        // f
        TokenTree::token_alone(TokenKind::Ident(sym::f, token::IdentIsRaw::No), span),
        TokenTree::token_alone(TokenKind::Comma, span),
        // "{}" or "{:?}"
        TokenTree::token_alone(
            TokenKind::Literal(Lit { kind: LitKind::Str, symbol: fmt_sym, suffix: None }),
            span,
        ),
        TokenTree::token_alone(TokenKind::Comma, span),
    ];

    // Add the value expression - we need to convert it to tokens
    // For simplicity, just use the identifier directly
    let val_tokens = expr_to_tokens(&val_expr, span);

    let mut all_tokens = tokens;
    all_tokens.extend(val_tokens);

    let args = Box::new(ast::DelimArgs {
        dspan: DelimSpan::from_single(span),
        delim: Delimiter::Parenthesis,
        tokens: TokenStream::new(all_tokens),
    });

    let write_path = ast::Path::from_ident(Ident::new(sym::write, span));
    let mac = Box::new(ast::MacCall { path: write_path, args });

    Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MacCall(mac),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    })
}

/// Build write!(f, "literal string") call
fn build_write_str_call(span: Span, _f_expr: Box<ast::Expr>, _str_expr: Box<ast::Expr>) -> Box<ast::Expr> {
    use rustc_ast::token::{self, Delimiter, Lit, LitKind, TokenKind};
    use rustc_ast::tokenstream::{DelimSpan, TokenStream, TokenTree};

    // Build token stream for write!(f, "{}", "nil")
    let tokens = vec![
        TokenTree::token_alone(TokenKind::Ident(sym::f, token::IdentIsRaw::No), span),
        TokenTree::token_alone(TokenKind::Comma, span),
        TokenTree::token_alone(
            TokenKind::Literal(Lit { kind: LitKind::Str, symbol: sym::empty_braces, suffix: None }),
            span,
        ),
        TokenTree::token_alone(TokenKind::Comma, span),
        TokenTree::token_alone(
            TokenKind::Literal(Lit { kind: LitKind::Str, symbol: sym::nil, suffix: None }),
            span,
        ),
    ];

    let args = Box::new(ast::DelimArgs {
        dspan: DelimSpan::from_single(span),
        delim: Delimiter::Parenthesis,
        tokens: TokenStream::new(tokens),
    });

    let write_path = ast::Path::from_ident(Ident::new(sym::write, span));
    let mac = Box::new(ast::MacCall { path: write_path, args });

    Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MacCall(mac),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    })
}

/// Convert simple expressions to tokens (for idents)
fn expr_to_tokens(expr: &ast::Expr, span: Span) -> Vec<rustc_ast::tokenstream::TokenTree> {
    use rustc_ast::token::{self, TokenKind};
    use rustc_ast::tokenstream::TokenTree;

    match &expr.kind {
        ast::ExprKind::Path(None, path) if path.segments.len() == 1 => {
            let ident = path.segments[0].ident;
            vec![TokenTree::token_alone(TokenKind::Ident(ident.name, token::IdentIsRaw::No), span)]
        }
        _ => vec![], // Fallback - shouldn't happen for our use cases
    }
}

/// Build impl From<&str> for Val
fn build_val_from_str_ref(def_site: Span, call_site: Span) -> Box<ast::Item> {
    // fn from(s: &str) -> Self { Val::Str(s.into()) }
    let body_expr = build_val_variant_call(call_site, sym::Str, build_into_call(call_site, sym::s));
    build_from_impl(def_site, call_site, build_str_ref_ty(call_site), sym::s, body_expr)
}

/// Build impl From<String> for Val
fn build_val_from_string(def_site: Span, call_site: Span) -> Box<ast::Item> {
    // fn from(s: String) -> Self { Val::Str(s) }
    let s_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::new(sym::s, call_site))),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });
    let body_expr = build_val_variant_call(call_site, sym::Str, s_expr);
    build_from_impl(def_site, call_site, build_simple_ty(call_site, sym::String), sym::s, body_expr)
}

/// Build impl From<i64> for Val
fn build_val_from_i64(def_site: Span, call_site: Span) -> Box<ast::Item> {
    let n_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::new(sym::__n, call_site))),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });
    let body_expr = build_val_variant_call(call_site, sym::Int, n_expr);
    build_from_impl(def_site, call_site, build_simple_ty(call_site, sym::i64), sym::__n, body_expr)
}

/// Build impl From<i32> for Val
fn build_val_from_i32(def_site: Span, call_site: Span) -> Box<ast::Item> {
    // fn from(n: i32) -> Self { Val::Int(n as i64) }
    let n_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::new(sym::__n, call_site))),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });
    let cast_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Cast(n_expr, build_simple_ty(call_site, sym::i64)),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });
    let body_expr = build_val_variant_call(call_site, sym::Int, cast_expr);
    build_from_impl(def_site, call_site, build_simple_ty(call_site, sym::i32), sym::__n, body_expr)
}

/// Build impl From<f64> for Val
fn build_val_from_f64(def_site: Span, call_site: Span) -> Box<ast::Item> {
    let n_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::new(sym::__n, call_site))),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });
    let body_expr = build_val_variant_call(call_site, sym::Float, n_expr);
    build_from_impl(def_site, call_site, build_simple_ty(call_site, sym::f64), sym::__n, body_expr)
}

/// Build impl From<f32> for Val
fn build_val_from_f32(def_site: Span, call_site: Span) -> Box<ast::Item> {
    // fn from(n: f32) -> Self { Val::Float(n as f64) }
    let n_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::new(sym::__n, call_site))),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });
    let cast_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Cast(n_expr, build_simple_ty(call_site, sym::f64)),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });
    let body_expr = build_val_variant_call(call_site, sym::Float, cast_expr);
    build_from_impl(def_site, call_site, build_simple_ty(call_site, sym::f32), sym::__n, body_expr)
}

/// Build impl From<bool> for Val
fn build_val_from_bool(def_site: Span, call_site: Span) -> Box<ast::Item> {
    let b_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::new(sym::__b, call_site))),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });
    let body_expr = build_val_variant_call(call_site, sym::Bool, b_expr);
    build_from_impl(def_site, call_site, build_simple_ty(call_site, sym::bool), sym::__b, body_expr)
}

/// Build Val::Variant(expr) expression
fn build_val_variant_call(span: Span, variant: rustc_span::Symbol, inner: Box<ast::Expr>) -> Box<ast::Expr> {
    let path = ast::Path {
        span,
        segments: ThinVec::from([
            ast::PathSegment::from_ident(Ident::new(sym::Val, span)),
            ast::PathSegment::from_ident(Ident::new(variant, span)),
        ]),
        tokens: None,
    };

    Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Call(
            Box::new(ast::Expr {
                id: ast::DUMMY_NODE_ID,
                kind: ast::ExprKind::Path(None, path),
                span,
                attrs: ThinVec::new(),
                tokens: None,
            }),
            ThinVec::from([inner]),
        ),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    })
}

/// Build expr.into() call
fn build_into_call(span: Span, binding: rustc_span::Symbol) -> Box<ast::Expr> {
    let receiver = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::new(binding, span))),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::into, span)),
            receiver,
            args: ThinVec::new(),
            span,
        })),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    })
}

/// Build a simple type like i32, String, bool
fn build_simple_ty(span: Span, name: rustc_span::Symbol) -> Box<ast::Ty> {
    Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(name, span))),
        span,
        tokens: None,
    })
}

/// Build &str type
fn build_str_ref_ty(span: Span) -> Box<ast::Ty> {
    Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Ref(
            None,
            ast::MutTy {
                ty: Box::new(ast::Ty {
                    id: ast::DUMMY_NODE_ID,
                    kind: ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(sym::str, span))),
                    span,
                    tokens: None,
                }),
                mutbl: ast::Mutability::Not,
            },
        ),
        span,
        tokens: None,
    })
}

/// Build impl From<T> for Val with given body
fn build_from_impl(
    def_site: Span,
    call_site: Span,
    from_ty: Box<ast::Ty>,
    param_name: rustc_span::Symbol,
    body_expr: Box<ast::Expr>,
) -> Box<ast::Item> {
    // fn from(param: T) -> Self { body }
    let fn_sig = ast::FnSig {
        decl: Box::new(ast::FnDecl {
            inputs: ThinVec::from([ast::Param {
                attrs: ThinVec::new(),
                ty: from_ty.clone(),
                pat: Box::new(ast::Pat {
                    id: ast::DUMMY_NODE_ID,
                    kind: ast::PatKind::Ident(
                        ast::BindingMode::NONE,
                        Ident::new(param_name, call_site),
                        None,
                    ),
                    span: call_site,
                    tokens: None,
                }),
                id: ast::DUMMY_NODE_ID,
                span: call_site,
                is_placeholder: false,
            }]),
            output: ast::FnRetTy::Ty(Box::new(ast::Ty {
                id: ast::DUMMY_NODE_ID,
                kind: ast::TyKind::ImplicitSelf,
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
        ident: Ident::new(sym::from, call_site),
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

    // Build From<T> trait path
    let from_path = ast::Path {
        span: call_site,
        segments: ThinVec::from([ast::PathSegment {
            ident: Ident::new(sym::From, call_site),
            id: ast::DUMMY_NODE_ID,
            args: Some(Box::new(ast::GenericArgs::AngleBracketed(ast::AngleBracketedArgs {
                span: call_site,
                args: ThinVec::from([ast::AngleBracketedArg::Arg(ast::GenericArg::Type(from_ty))]),
            }))),
        }]),
        tokens: None,
    };

    let val_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(sym::Val, call_site))),
        span: call_site,
        tokens: None,
    });

    let impl_def = ast::Impl {
        generics: ast::Generics::default(),
        constness: ast::Const::No,
        of_trait: Some(Box::new(ast::TraitImplHeader {
            defaultness: ast::Defaultness::Final,
            safety: ast::Safety::Default,
            polarity: ast::ImplPolarity::Positive,
            trait_ref: ast::TraitRef { path: from_path, ref_id: ast::DUMMY_NODE_ID },
        })),
        self_ty: val_ty,
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

/// Build impl Truthy for Val
fn build_val_truthy_impl(
    def_site: Span,
    call_site: Span,
    trait_path: &ast::Path,
) -> Box<ast::Item> {
    // Build match expression for is_truthy:
    // match self {
    //     Val::Nil => false,
    //     Val::Bool(b) => *b,
    //     Val::Int(n) => *n != 0,
    //     Val::Float(f) => *f != 0.0,
    //     Val::Str(s) => !s.is_empty(),
    //     Val::List(v) => !v.is_empty(),
    // }

    let self_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(
            Ident::with_dummy_span(kw::SelfLower).with_span_pos(call_site)
        )),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let arms = ThinVec::from([
        build_truthy_nil_arm(call_site),
        build_truthy_bool_arm(call_site),
        build_truthy_int_arm(call_site),
        build_truthy_float_arm(call_site),
        build_truthy_str_arm(call_site),
        build_truthy_list_arm(call_site),
    ]);

    let match_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Match(self_expr, arms, ast::MatchKind::Prefix),
        span: call_site,
        attrs: ThinVec::new(),
        tokens: None,
    });

    // Build the is_truthy method
    let fn_sig = ast::FnSig {
        decl: Box::new(ast::FnDecl {
            inputs: ThinVec::from([ast::Param {
                attrs: ThinVec::new(),
                ty: Box::new(ast::Ty {
                    id: ast::DUMMY_NODE_ID,
                    kind: ast::TyKind::Ref(
                        None,
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
            kind: ast::StmtKind::Expr(match_expr),
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

    let val_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(sym::Val, call_site))),
        span: call_site,
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
        self_ty: val_ty,
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

/// Build arm: Val::Nil => false
fn build_truthy_nil_arm(span: Span) -> ast::Arm {
    let pat_path = ast::Path {
        span,
        segments: ThinVec::from([
            ast::PathSegment::from_ident(Ident::new(sym::Val, span)),
            ast::PathSegment::from_ident(Ident::new(sym::Nil, span)),
        ]),
        tokens: None,
    };

    let pat = Box::new(ast::Pat {
        id: ast::DUMMY_NODE_ID,
        kind: ast::PatKind::Path(None, pat_path),
        span,
        tokens: None,
    });

    let body = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Lit(rustc_ast::token::Lit {
            kind: rustc_ast::token::LitKind::Bool,
            symbol: kw::False,
            suffix: None,
        }),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    ast::Arm {
        attrs: ThinVec::new(),
        pat,
        guard: None,
        body: Some(body),
        span,
        id: ast::DUMMY_NODE_ID,
        is_placeholder: false,
    }
}

/// Build arm: Val::Bool(b) => *b
fn build_truthy_bool_arm(span: Span) -> ast::Arm {
    let pat_path = ast::Path {
        span,
        segments: ThinVec::from([
            ast::PathSegment::from_ident(Ident::new(sym::Val, span)),
            ast::PathSegment::from_ident(Ident::new(sym::Bool, span)),
        ]),
        tokens: None,
    };

    let binding_pat = ast::Pat {
        id: ast::DUMMY_NODE_ID,
        kind: ast::PatKind::Ident(ast::BindingMode::NONE, Ident::new(sym::__b, span), None),
        span,
        tokens: None,
    };

    let pat = Box::new(ast::Pat {
        id: ast::DUMMY_NODE_ID,
        kind: ast::PatKind::TupleStruct(None, pat_path, ThinVec::from([binding_pat])),
        span,
        tokens: None,
    });

    // *__b
    let body = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Unary(
            ast::UnOp::Deref,
            Box::new(ast::Expr {
                id: ast::DUMMY_NODE_ID,
                kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::new(sym::__b, span))),
                span,
                attrs: ThinVec::new(),
                tokens: None,
            }),
        ),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    ast::Arm {
        attrs: ThinVec::new(),
        pat,
        guard: None,
        body: Some(body),
        span,
        id: ast::DUMMY_NODE_ID,
        is_placeholder: false,
    }
}

/// Build arm: Val::Int(n) => *n != 0
fn build_truthy_int_arm(span: Span) -> ast::Arm {
    let pat_path = ast::Path {
        span,
        segments: ThinVec::from([
            ast::PathSegment::from_ident(Ident::new(sym::Val, span)),
            ast::PathSegment::from_ident(Ident::new(sym::Int, span)),
        ]),
        tokens: None,
    };

    let binding_pat = ast::Pat {
        id: ast::DUMMY_NODE_ID,
        kind: ast::PatKind::Ident(ast::BindingMode::NONE, Ident::new(sym::__n, span), None),
        span,
        tokens: None,
    };

    let pat = Box::new(ast::Pat {
        id: ast::DUMMY_NODE_ID,
        kind: ast::PatKind::TupleStruct(None, pat_path, ThinVec::from([binding_pat])),
        span,
        tokens: None,
    });

    // *__n != 0
    let deref_n = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Unary(
            ast::UnOp::Deref,
            Box::new(ast::Expr {
                id: ast::DUMMY_NODE_ID,
                kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::new(sym::__n, span))),
                span,
                attrs: ThinVec::new(),
                tokens: None,
            }),
        ),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let zero = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Lit(rustc_ast::token::Lit {
            kind: rustc_ast::token::LitKind::Integer,
            symbol: sym::integer(0),
            suffix: None,
        }),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let body = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Binary(
            rustc_span::source_map::Spanned { node: ast::BinOpKind::Ne, span },
            deref_n,
            zero,
        ),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    ast::Arm {
        attrs: ThinVec::new(),
        pat,
        guard: None,
        body: Some(body),
        span,
        id: ast::DUMMY_NODE_ID,
        is_placeholder: false,
    }
}

/// Build arm: Val::Float(f) => *f != 0.0
fn build_truthy_float_arm(span: Span) -> ast::Arm {
    let pat_path = ast::Path {
        span,
        segments: ThinVec::from([
            ast::PathSegment::from_ident(Ident::new(sym::Val, span)),
            ast::PathSegment::from_ident(Ident::new(sym::Float, span)),
        ]),
        tokens: None,
    };

    let binding_pat = ast::Pat {
        id: ast::DUMMY_NODE_ID,
        kind: ast::PatKind::Ident(ast::BindingMode::NONE, Ident::new(sym::f, span), None),
        span,
        tokens: None,
    };

    let pat = Box::new(ast::Pat {
        id: ast::DUMMY_NODE_ID,
        kind: ast::PatKind::TupleStruct(None, pat_path, ThinVec::from([binding_pat])),
        span,
        tokens: None,
    });

    // *f != 0.0
    let deref_f = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Unary(
            ast::UnOp::Deref,
            Box::new(ast::Expr {
                id: ast::DUMMY_NODE_ID,
                kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::new(sym::f, span))),
                span,
                attrs: ThinVec::new(),
                tokens: None,
            }),
        ),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let zero = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Lit(rustc_ast::token::Lit {
            kind: rustc_ast::token::LitKind::Float,
            symbol: sym::float_zero,
            suffix: None,
        }),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let body = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Binary(
            rustc_span::source_map::Spanned { node: ast::BinOpKind::Ne, span },
            deref_f,
            zero,
        ),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    ast::Arm {
        attrs: ThinVec::new(),
        pat,
        guard: None,
        body: Some(body),
        span,
        id: ast::DUMMY_NODE_ID,
        is_placeholder: false,
    }
}

/// Build arm: Val::Str(s) => !s.is_empty()
fn build_truthy_str_arm(span: Span) -> ast::Arm {
    let pat_path = ast::Path {
        span,
        segments: ThinVec::from([
            ast::PathSegment::from_ident(Ident::new(sym::Val, span)),
            ast::PathSegment::from_ident(Ident::new(sym::Str, span)),
        ]),
        tokens: None,
    };

    let binding_pat = ast::Pat {
        id: ast::DUMMY_NODE_ID,
        kind: ast::PatKind::Ident(ast::BindingMode::NONE, Ident::new(sym::s, span), None),
        span,
        tokens: None,
    };

    let pat = Box::new(ast::Pat {
        id: ast::DUMMY_NODE_ID,
        kind: ast::PatKind::TupleStruct(None, pat_path, ThinVec::from([binding_pat])),
        span,
        tokens: None,
    });

    // !s.is_empty()
    let s_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::new(sym::s, span))),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let is_empty_call = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::is_empty, span)),
            receiver: s_expr,
            args: ThinVec::new(),
            span,
        })),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let body = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Unary(ast::UnOp::Not, is_empty_call),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    ast::Arm {
        attrs: ThinVec::new(),
        pat,
        guard: None,
        body: Some(body),
        span,
        id: ast::DUMMY_NODE_ID,
        is_placeholder: false,
    }
}

/// Build arm: Val::List(v) => !v.is_empty()
fn build_truthy_list_arm(span: Span) -> ast::Arm {
    let pat_path = ast::Path {
        span,
        segments: ThinVec::from([
            ast::PathSegment::from_ident(Ident::new(sym::Val, span)),
            ast::PathSegment::from_ident(Ident::new(sym::List, span)),
        ]),
        tokens: None,
    };

    let binding_pat = ast::Pat {
        id: ast::DUMMY_NODE_ID,
        kind: ast::PatKind::Ident(ast::BindingMode::NONE, Ident::new(sym::__v, span), None),
        span,
        tokens: None,
    };

    let pat = Box::new(ast::Pat {
        id: ast::DUMMY_NODE_ID,
        kind: ast::PatKind::TupleStruct(None, pat_path, ThinVec::from([binding_pat])),
        span,
        tokens: None,
    });

    // !__v.is_empty()
    let v_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::new(sym::__v, span))),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let is_empty_call = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(sym::is_empty, span)),
            receiver: v_expr,
            args: ThinVec::new(),
            span,
        })),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    let body = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Unary(ast::UnOp::Not, is_empty_call),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });

    ast::Arm {
        attrs: ThinVec::new(),
        pat,
        guard: None,
        body: Some(body),
        span,
        id: ast::DUMMY_NODE_ID,
        is_placeholder: false,
    }
}
