//! Truthy trait implementation for script mode.
//!
//! Generates a `Truthy` trait that allows values to be used in boolean contexts,
//! similar to JavaScript/Python truthiness.

use rustc_ast as ast;
use rustc_span::{Ident, Span, kw, sym};
use thin_vec::ThinVec;

use super::create_allow_attr;

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
pub fn build_truthy_helpers(def_site: Span, call_site: Span) -> ThinVec<Box<ast::Item>> {
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
