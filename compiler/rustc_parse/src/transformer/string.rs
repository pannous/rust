//! String helper traits and implementations for script mode.
//!
//! Generates ScriptStrExt trait with methods like first(), last(), reverse(), size(), length().

use rustc_ast as ast;
use rustc_span::{Ident, Span, kw, sym};
use thin_vec::ThinVec;

use super::create_allow_attr;

/// Build ScriptStrExt trait and impl for &str.
/// Generates:
/// ```ignore
/// trait ScriptStrExt {
///     fn first(&self) -> String;
///     fn last(&self) -> String;
///     fn reverse(&self) -> String;
///     fn size(&self) -> usize;
///     fn length(&self) -> usize;
/// }
/// impl ScriptStrExt for &str { ... }
/// ```
pub fn build_string_helpers(def_site: Span, call_site: Span) -> ThinVec<Box<ast::Item>> {
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
