//! Slice filter extension trait generator for script mode.
//!
//! Generates a `ScriptSliceExt` trait with a `.filter()` method that can be
//! called on slices, providing a more convenient syntax than the standard
//! iterator-based approach.

use rustc_ast as ast;
use rustc_span::{Ident, Span, kw, sym};
use thin_vec::ThinVec;

/// Create #[allow(lint_name)] attribute
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
pub(crate) fn build_slice_helpers(def_site: Span, call_site: Span) -> ThinVec<Box<ast::Item>> {
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
