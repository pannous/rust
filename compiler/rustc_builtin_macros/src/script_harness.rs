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

use rustc_parse::transformer;

/// Inject script mode helpers and optionally wrap in main.
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

    // Set up expansion context for proper hygiene (like standard_library_imports does)
    let expn_id = resolver.expansion_for_ast_pass(
        DUMMY_SP,
        AstPass::ScriptMain,
        &[],
        None,
    );
    let def_site = DUMMY_SP.with_def_site_ctxt(expn_id.to_expn_id());
    let call_site = DUMMY_SP.with_call_site_ctxt(expn_id.to_expn_id());

    // Check if file already has a main function
    let has_main = has_entry_point(krate);

    // Always inject helpers in script mode, optionally wrap in main
    inject_helpers(krate, def_site, call_site, has_main);
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

/// Inject script mode helpers and optionally generate main function.
fn inject_helpers(krate: &mut ast::Crate, def_site: Span, call_site: Span, has_main: bool) {
    // Build items with proper hygiene contexts:
    // - def_site: for internal macro implementation (invisible to user)
    // - call_site: for macro names (visible to user code)
    let type_aliases = build_type_aliases(call_site);
    let script_macros = transformer::build_script_macros(def_site, call_site);
    let string_helpers = transformer::build_string_helpers(def_site, call_site);
    let slice_helpers = transformer::build_slice_ext(def_site, call_site);
    let truthy_helpers = transformer::build_truthy_helpers(def_site, call_site);
    let val_helpers = transformer::build_val_helpers(def_site, call_site);
    let exit_fn = transformer::build_exit_function(def_site, call_site);
    let approx_eq_fn = transformer::build_approx_eq_function(def_site, call_site);
    let math_constants = transformer::build_math_constants(def_site, call_site);

    // Partition items and optionally build main
    let (module_items, main_stmts) = partition_items(&krate.items);

    // Rebuild crate: helpers first, then module items
    krate.items = type_aliases;
    krate.items.extend(script_macros);
    krate.items.extend(string_helpers);
    krate.items.extend(slice_helpers);
    krate.items.extend(truthy_helpers);
    krate.items.extend(val_helpers);
    krate.items.push(exit_fn);
    krate.items.push(approx_eq_fn);
    krate.items.extend(math_constants);
    krate.items.extend(module_items);

    // Only generate main if file doesn't have one
    if !has_main && !main_stmts.is_empty() {
        let main_fn = build_main(def_site, main_stmts);
        krate.items.push(main_fn);
    }
}

/// Build type aliases for script mode: type int = i64; type float = f64;
fn build_type_aliases(span: Span) -> ThinVec<Box<ast::Item>> {
    use rustc_span::kw;

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
                ty: Some(transformer::build_simple_ty(span, target)),
            })),
            vis: ast::Visibility { span, kind: ast::VisibilityKind::Inherited, tokens: None },
            span,
            tokens: None,
        })
    };

    // Build type string = &'static str
    let make_str_ref_alias = || -> Box<ast::Item> {
        let str_ref_ty = Box::new(ast::Ty {
            id: ast::DUMMY_NODE_ID,
            kind: ast::TyKind::Ref(
                Some(ast::Lifetime {
                    id: ast::DUMMY_NODE_ID,
                    ident: Ident::new(kw::StaticLifetime, span),
                }),
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
        });
        Box::new(ast::Item {
            attrs: ThinVec::new(),
            id: ast::DUMMY_NODE_ID,
            kind: ast::ItemKind::TyAlias(Box::new(ast::TyAlias {
                defaultness: ast::Defaultness::Final,
                ident: Ident::new(sym::string, span),
                generics: ast::Generics::default(),
                after_where_clause: ast::WhereClause {
                    has_where_token: false,
                    predicates: ThinVec::new(),
                    span,
                },
                bounds: Vec::new(),
                ty: Some(str_ref_ty),
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
    items.push(make_str_ref_alias());

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

// String helpers moved to transformer/string.rs

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
    let allow_unused_variables = create_allow_attr(span, sym::unused_variables);

    // Node IDs will be assigned during macro expansion
    Box::new(ast::Item {
        attrs: vec![allow_unused_mut, allow_unused_variables].into(),
        id: ast::DUMMY_NODE_ID,
        kind: main_fn,
        vis: ast::Visibility { span, kind: ast::VisibilityKind::Public, tokens: None },
        span,
        tokens: None,
    })
}
