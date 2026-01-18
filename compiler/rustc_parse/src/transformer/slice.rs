//! Slice extension methods for script mode: map, filter synonyms.
//!
//! Generates:
//! ```ignore
//! trait ScriptSliceExt<T: Clone> {
//!     fn mapped<U, F: Fn(T) -> U>(&self, f: F) -> Vec<U>;  // + apply, transform, convert
//!     fn filtered<F: Fn(&T) -> bool>(&self, f: F) -> Vec<T>;  // + select, chose, that, which
//! }
//! impl<T: Clone, S: AsRef<[T]>> ScriptSliceExt<T> for S { ... }
//! ```

use rustc_ast as ast;
use rustc_span::{Ident, Span, kw, sym};
use thin_vec::ThinVec;

use super::create_allow_attr;

/// Build slice helper trait and impl.
pub fn build_slice_helpers(def_site: Span, call_site: Span) -> ThinVec<Box<ast::Item>> {
    let mut items = ThinVec::new();

    let allow_dead_code = create_allow_attr(def_site, sym::dead_code);

    let trait_name = sym::ScriptSliceExt;
    let t_ident = Ident::new(sym::T, call_site);

    // Clone bound for T
    let clone_bound = ast::GenericBound::Trait(ast::PolyTraitRef {
        bound_generic_params: ThinVec::new(),
        modifiers: ast::TraitBoundModifiers::NONE,
        trait_ref: ast::TraitRef {
            path: ast::Path::from_ident(Ident::new(sym::Clone, call_site)),
            ref_id: ast::DUMMY_NODE_ID,
        },
        span: call_site,
        parens: ast::Parens::No,
    });

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

    // Build trait methods - mapped and synonyms
    let mut trait_items = ThinVec::new();
    trait_items.push(build_map_trait_item(call_site, t_ident, sym::mapped));
    trait_items.push(build_map_trait_item(call_site, t_ident, sym::apply));
    trait_items.push(build_map_trait_item(call_site, t_ident, sym::transform));
    trait_items.push(build_map_trait_item(call_site, t_ident, sym::convert));
    // filtered and synonyms
    trait_items.push(build_filter_trait_item(call_site, t_ident, sym::filtered));
    trait_items.push(build_filter_trait_item(call_site, t_ident, sym::select));
    trait_items.push(build_filter_trait_item(call_site, t_ident, sym::chose));
    trait_items.push(build_filter_trait_item(call_site, t_ident, sym::that));
    trait_items.push(build_filter_trait_item(call_site, t_ident, sym::which));

    let trait_def = ast::Trait {
        constness: ast::Const::No,
        safety: ast::Safety::Default,
        is_auto: ast::IsAuto::No,
        ident: Ident::new(trait_name, call_site),
        generics: trait_generics,
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

    // Build blanket impl for anything that AsRef<[T]>
    items.push(build_blanket_impl(
        def_site,
        call_site,
        trait_name,
        t_ident,
        clone_bound,
    ));

    items
}

/// Build: fn <name><U, F: Fn(T) -> U>(&self, f: F) -> Vec<U>;
fn build_map_trait_item(
    span: Span,
    t_ident: Ident,
    method_name: rustc_span::Symbol,
) -> Box<ast::AssocItem> {
    let u_ident = Ident::new(sym::U, span);
    let f_ident = Ident::new(sym::F, span);

    let t_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(None, ast::Path::from_ident(t_ident)),
        span,
        tokens: None,
    });

    let u_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(None, ast::Path::from_ident(u_ident)),
        span,
        tokens: None,
    });

    let fn_bound = ast::GenericBound::Trait(ast::PolyTraitRef {
        bound_generic_params: ThinVec::new(),
        modifiers: ast::TraitBoundModifiers::NONE,
        trait_ref: ast::TraitRef {
            path: ast::Path {
                span,
                segments: ThinVec::from([ast::PathSegment {
                    ident: Ident::new(sym::Fn, span),
                    id: ast::DUMMY_NODE_ID,
                    args: Some(Box::new(ast::GenericArgs::Parenthesized(
                        ast::ParenthesizedArgs {
                            span,
                            inputs: ThinVec::from([t_ty.clone()]),
                            inputs_span: span,
                            output: ast::FnRetTy::Ty(u_ty.clone()),
                        },
                    ))),
                }]),
                tokens: None,
            },
            ref_id: ast::DUMMY_NODE_ID,
        },
        span,
        parens: ast::Parens::No,
    });

    let u_param = ast::GenericParam {
        id: ast::DUMMY_NODE_ID,
        ident: u_ident,
        attrs: ThinVec::new(),
        bounds: vec![],
        is_placeholder: false,
        kind: ast::GenericParamKind::Type { default: None },
        colon_span: None,
    };

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
        params: ThinVec::from([u_param, f_param]),
        where_clause: ast::WhereClause {
            has_where_token: false,
            predicates: ThinVec::new(),
            span,
        },
        span,
    };

    let vec_u_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(
            None,
            ast::Path {
                span,
                segments: ThinVec::from([ast::PathSegment {
                    ident: Ident::new(sym::Vec, span),
                    id: ast::DUMMY_NODE_ID,
                    args: Some(Box::new(ast::GenericArgs::AngleBracketed(
                        ast::AngleBracketedArgs {
                            span,
                            args: ThinVec::from([ast::AngleBracketedArg::Arg(
                                ast::GenericArg::Type(u_ty),
                            )]),
                        },
                    ))),
                }]),
                tokens: None,
            },
        ),
        span,
        tokens: None,
    });

    let self_param = build_self_param(span);
    let f_param_decl = ast::Param {
        attrs: ThinVec::new(),
        ty: Box::new(ast::Ty {
            id: ast::DUMMY_NODE_ID,
            kind: ast::TyKind::Path(None, ast::Path::from_ident(f_ident)),
            span,
            tokens: None,
        }),
        pat: Box::new(ast::Pat {
            id: ast::DUMMY_NODE_ID,
            kind: ast::PatKind::Ident(ast::BindingMode::NONE, Ident::new(sym::f, span), None),
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
            output: ast::FnRetTy::Ty(vec_u_ty),
        }),
        span,
    };

    Box::new(ast::AssocItem {
        attrs: ThinVec::new(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::AssocItemKind::Fn(Box::new(ast::Fn {
            defaultness: ast::Defaultness::Final,
            ident: Ident::new(method_name, span),
            generics: method_generics,
            sig: fn_sig,
            contract: None,
            body: None,
            define_opaque: None,
            eii_impls: ThinVec::new(),
        })),
        vis: ast::Visibility { span, kind: ast::VisibilityKind::Inherited, tokens: None },
        span,
        tokens: None,
    })
}

/// Build: fn <name><F: Fn(&T) -> bool>(&self, f: F) -> Vec<T>;
fn build_filter_trait_item(
    span: Span,
    t_ident: Ident,
    method_name: rustc_span::Symbol,
) -> Box<ast::AssocItem> {
    let f_ident = Ident::new(sym::F, span);

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

    let bool_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(sym::bool, span))),
        span,
        tokens: None,
    });

    let fn_bound = ast::GenericBound::Trait(ast::PolyTraitRef {
        bound_generic_params: ThinVec::new(),
        modifiers: ast::TraitBoundModifiers::NONE,
        trait_ref: ast::TraitRef {
            path: ast::Path {
                span,
                segments: ThinVec::from([ast::PathSegment {
                    ident: Ident::new(sym::Fn, span),
                    id: ast::DUMMY_NODE_ID,
                    args: Some(Box::new(ast::GenericArgs::Parenthesized(
                        ast::ParenthesizedArgs {
                            span,
                            inputs: ThinVec::from([t_ref_ty]),
                            inputs_span: span,
                            output: ast::FnRetTy::Ty(bool_ty),
                        },
                    ))),
                }]),
                tokens: None,
            },
            ref_id: ast::DUMMY_NODE_ID,
        },
        span,
        parens: ast::Parens::No,
    });

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
                    args: Some(Box::new(ast::GenericArgs::AngleBracketed(
                        ast::AngleBracketedArgs {
                            span,
                            args: ThinVec::from([ast::AngleBracketedArg::Arg(
                                ast::GenericArg::Type(t_ty),
                            )]),
                        },
                    ))),
                }]),
                tokens: None,
            },
        ),
        span,
        tokens: None,
    });

    let self_param = build_self_param(span);
    let f_param_decl = ast::Param {
        attrs: ThinVec::new(),
        ty: Box::new(ast::Ty {
            id: ast::DUMMY_NODE_ID,
            kind: ast::TyKind::Path(None, ast::Path::from_ident(f_ident)),
            span,
            tokens: None,
        }),
        pat: Box::new(ast::Pat {
            id: ast::DUMMY_NODE_ID,
            kind: ast::PatKind::Ident(ast::BindingMode::NONE, Ident::new(sym::f, span), None),
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
            ident: Ident::new(method_name, span),
            generics: method_generics,
            sig: fn_sig,
            contract: None,
            body: None,
            define_opaque: None,
            eii_impls: ThinVec::new(),
        })),
        vis: ast::Visibility { span, kind: ast::VisibilityKind::Inherited, tokens: None },
        span,
        tokens: None,
    })
}

/// Build impl<T: Clone, S: AsRef<[T]>> ScriptSliceExt<T> for S
fn build_blanket_impl(
    def_site: Span,
    call_site: Span,
    trait_name: rustc_span::Symbol,
    t_ident: Ident,
    clone_bound: ast::GenericBound,
) -> Box<ast::Item> {
    let s_ident = Ident::new(sym::S, call_site);

    let t_param = ast::GenericParam {
        id: ast::DUMMY_NODE_ID,
        ident: t_ident,
        attrs: ThinVec::new(),
        bounds: vec![clone_bound],
        is_placeholder: false,
        kind: ast::GenericParamKind::Type { default: None },
        colon_span: None,
    };

    let t_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(None, ast::Path::from_ident(t_ident)),
        span: call_site,
        tokens: None,
    });

    let slice_t_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Slice(t_ty.clone()),
        span: call_site,
        tokens: None,
    });

    let asref_bound = ast::GenericBound::Trait(ast::PolyTraitRef {
        bound_generic_params: ThinVec::new(),
        modifiers: ast::TraitBoundModifiers::NONE,
        trait_ref: ast::TraitRef {
            path: ast::Path {
                span: call_site,
                segments: ThinVec::from([ast::PathSegment {
                    ident: Ident::new(sym::AsRef, call_site),
                    id: ast::DUMMY_NODE_ID,
                    args: Some(Box::new(ast::GenericArgs::AngleBracketed(
                        ast::AngleBracketedArgs {
                            span: call_site,
                            args: ThinVec::from([ast::AngleBracketedArg::Arg(
                                ast::GenericArg::Type(slice_t_ty),
                            )]),
                        },
                    ))),
                }]),
                tokens: None,
            },
            ref_id: ast::DUMMY_NODE_ID,
        },
        span: call_site,
        parens: ast::Parens::No,
    });

    let s_param = ast::GenericParam {
        id: ast::DUMMY_NODE_ID,
        ident: s_ident,
        attrs: ThinVec::new(),
        bounds: vec![asref_bound],
        is_placeholder: false,
        kind: ast::GenericParamKind::Type { default: None },
        colon_span: None,
    };

    let impl_generics = ast::Generics {
        params: ThinVec::from([t_param, s_param]),
        where_clause: ast::WhereClause {
            has_where_token: false,
            predicates: ThinVec::new(),
            span: call_site,
        },
        span: call_site,
    };

    let s_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(None, ast::Path::from_ident(s_ident)),
        span: call_site,
        tokens: None,
    });

    let trait_path = ast::Path {
        span: call_site,
        segments: ThinVec::from([ast::PathSegment {
            ident: Ident::new(trait_name, call_site),
            id: ast::DUMMY_NODE_ID,
            args: Some(Box::new(ast::GenericArgs::AngleBracketed(
                ast::AngleBracketedArgs {
                    span: call_site,
                    args: ThinVec::from([ast::AngleBracketedArg::Arg(ast::GenericArg::Type(t_ty))]),
                },
            ))),
        }]),
        tokens: None,
    };

    let mut impl_items = ThinVec::new();
    impl_items.push(build_map_impl_item(call_site, t_ident, sym::mapped));
    impl_items.push(build_map_impl_item(call_site, t_ident, sym::apply));
    impl_items.push(build_map_impl_item(call_site, t_ident, sym::transform));
    impl_items.push(build_map_impl_item(call_site, t_ident, sym::convert));
    impl_items.push(build_filter_impl_item(call_site, t_ident, sym::filtered));
    impl_items.push(build_filter_impl_item(call_site, t_ident, sym::select));
    impl_items.push(build_filter_impl_item(call_site, t_ident, sym::chose));
    impl_items.push(build_filter_impl_item(call_site, t_ident, sym::that));
    impl_items.push(build_filter_impl_item(call_site, t_ident, sym::which));

    let impl_def = ast::Impl {
        generics: impl_generics,
        constness: ast::Const::No,
        of_trait: Some(Box::new(ast::TraitImplHeader {
            defaultness: ast::Defaultness::Final,
            safety: ast::Safety::Default,
            polarity: ast::ImplPolarity::Positive,
            trait_ref: ast::TraitRef { path: trait_path, ref_id: ast::DUMMY_NODE_ID },
        })),
        self_ty: s_ty,
        items: impl_items,
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

/// Build map impl: self.as_ref().iter().cloned().map(f).collect()
fn build_map_impl_item(
    span: Span,
    t_ident: Ident,
    method_name: rustc_span::Symbol,
) -> Box<ast::AssocItem> {
    let u_ident = Ident::new(sym::U, span);
    let f_ident = Ident::new(sym::F, span);

    let t_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(None, ast::Path::from_ident(t_ident)),
        span,
        tokens: None,
    });
    let u_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(None, ast::Path::from_ident(u_ident)),
        span,
        tokens: None,
    });

    let fn_bound = ast::GenericBound::Trait(ast::PolyTraitRef {
        bound_generic_params: ThinVec::new(),
        modifiers: ast::TraitBoundModifiers::NONE,
        trait_ref: ast::TraitRef {
            path: ast::Path {
                span,
                segments: ThinVec::from([ast::PathSegment {
                    ident: Ident::new(sym::Fn, span),
                    id: ast::DUMMY_NODE_ID,
                    args: Some(Box::new(ast::GenericArgs::Parenthesized(
                        ast::ParenthesizedArgs {
                            span,
                            inputs: ThinVec::from([t_ty]),
                            inputs_span: span,
                            output: ast::FnRetTy::Ty(u_ty.clone()),
                        },
                    ))),
                }]),
                tokens: None,
            },
            ref_id: ast::DUMMY_NODE_ID,
        },
        span,
        parens: ast::Parens::No,
    });

    let u_param = ast::GenericParam {
        id: ast::DUMMY_NODE_ID,
        ident: u_ident,
        attrs: ThinVec::new(),
        bounds: vec![],
        is_placeholder: false,
        kind: ast::GenericParamKind::Type { default: None },
        colon_span: None,
    };
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
        params: ThinVec::from([u_param, f_param]),
        where_clause: ast::WhereClause {
            has_where_token: false,
            predicates: ThinVec::new(),
            span,
        },
        span,
    };

    let vec_u_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(
            None,
            ast::Path {
                span,
                segments: ThinVec::from([ast::PathSegment {
                    ident: Ident::new(sym::Vec, span),
                    id: ast::DUMMY_NODE_ID,
                    args: Some(Box::new(ast::GenericArgs::AngleBracketed(
                        ast::AngleBracketedArgs {
                            span,
                            args: ThinVec::from([ast::AngleBracketedArg::Arg(
                                ast::GenericArg::Type(u_ty),
                            )]),
                        },
                    ))),
                }]),
                tokens: None,
            },
        ),
        span,
        tokens: None,
    });

    let self_param = build_self_param(span);
    let f_param_decl = ast::Param {
        attrs: ThinVec::new(),
        ty: Box::new(ast::Ty {
            id: ast::DUMMY_NODE_ID,
            kind: ast::TyKind::Path(None, ast::Path::from_ident(f_ident)),
            span,
            tokens: None,
        }),
        pat: Box::new(ast::Pat {
            id: ast::DUMMY_NODE_ID,
            kind: ast::PatKind::Ident(ast::BindingMode::NONE, Ident::new(sym::f, span), None),
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
            output: ast::FnRetTy::Ty(vec_u_ty),
        }),
        span,
    };

    let body = build_map_body(span);

    Box::new(ast::AssocItem {
        attrs: ThinVec::new(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::AssocItemKind::Fn(Box::new(ast::Fn {
            defaultness: ast::Defaultness::Final,
            ident: Ident::new(method_name, span),
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

/// Build filter impl: self.as_ref().iter().filter(|x| f(x)).cloned().collect()
fn build_filter_impl_item(
    span: Span,
    t_ident: Ident,
    method_name: rustc_span::Symbol,
) -> Box<ast::AssocItem> {
    let f_ident = Ident::new(sym::F, span);

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
    let bool_ty = Box::new(ast::Ty {
        id: ast::DUMMY_NODE_ID,
        kind: ast::TyKind::Path(None, ast::Path::from_ident(Ident::new(sym::bool, span))),
        span,
        tokens: None,
    });

    let fn_bound = ast::GenericBound::Trait(ast::PolyTraitRef {
        bound_generic_params: ThinVec::new(),
        modifiers: ast::TraitBoundModifiers::NONE,
        trait_ref: ast::TraitRef {
            path: ast::Path {
                span,
                segments: ThinVec::from([ast::PathSegment {
                    ident: Ident::new(sym::Fn, span),
                    id: ast::DUMMY_NODE_ID,
                    args: Some(Box::new(ast::GenericArgs::Parenthesized(
                        ast::ParenthesizedArgs {
                            span,
                            inputs: ThinVec::from([t_ref_ty]),
                            inputs_span: span,
                            output: ast::FnRetTy::Ty(bool_ty),
                        },
                    ))),
                }]),
                tokens: None,
            },
            ref_id: ast::DUMMY_NODE_ID,
        },
        span,
        parens: ast::Parens::No,
    });

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
                    args: Some(Box::new(ast::GenericArgs::AngleBracketed(
                        ast::AngleBracketedArgs {
                            span,
                            args: ThinVec::from([ast::AngleBracketedArg::Arg(
                                ast::GenericArg::Type(t_ty),
                            )]),
                        },
                    ))),
                }]),
                tokens: None,
            },
        ),
        span,
        tokens: None,
    });

    let self_param = build_self_param(span);
    let f_param_decl = ast::Param {
        attrs: ThinVec::new(),
        ty: Box::new(ast::Ty {
            id: ast::DUMMY_NODE_ID,
            kind: ast::TyKind::Path(None, ast::Path::from_ident(f_ident)),
            span,
            tokens: None,
        }),
        pat: Box::new(ast::Pat {
            id: ast::DUMMY_NODE_ID,
            kind: ast::PatKind::Ident(ast::BindingMode::NONE, Ident::new(sym::f, span), None),
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

    let body = build_filter_body(span);

    Box::new(ast::AssocItem {
        attrs: ThinVec::new(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::AssocItemKind::Fn(Box::new(ast::Fn {
            defaultness: ast::Defaultness::Final,
            ident: Ident::new(method_name, span),
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

fn build_self_param(span: Span) -> ast::Param {
    ast::Param {
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
    }
}

fn build_method_call(
    receiver: Box<ast::Expr>,
    method: rustc_span::Symbol,
    args: ThinVec<Box<ast::Expr>>,
    span: Span,
) -> Box<ast::Expr> {
    Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::MethodCall(Box::new(ast::MethodCall {
            seg: ast::PathSegment::from_ident(Ident::new(method, span)),
            receiver,
            args,
            span,
        })),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    })
}

/// Build: self.as_ref().iter().cloned().map(f).collect()
fn build_map_body(span: Span) -> Box<ast::Block> {
    let self_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::new(kw::SelfLower, span))),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });
    let as_ref_expr = build_method_call(self_expr, sym::as_ref, ThinVec::new(), span);
    let iter_expr = build_method_call(as_ref_expr, sym::iter, ThinVec::new(), span);
    let cloned_expr = build_method_call(iter_expr, sym::cloned, ThinVec::new(), span);
    let f_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::new(sym::f, span))),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });
    let map_expr = build_method_call(cloned_expr, sym::map, ThinVec::from([f_expr]), span);
    let collect_expr = build_method_call(map_expr, sym::collect, ThinVec::new(), span);

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

/// Build: self.as_ref().iter().filter(|x| f(x)).cloned().collect()
fn build_filter_body(span: Span) -> Box<ast::Block> {
    let self_expr = Box::new(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        kind: ast::ExprKind::Path(None, ast::Path::from_ident(Ident::new(kw::SelfLower, span))),
        span,
        attrs: ThinVec::new(),
        tokens: None,
    });
    let as_ref_expr = build_method_call(self_expr, sym::as_ref, ThinVec::new(), span);
    let iter_expr = build_method_call(as_ref_expr, sym::iter, ThinVec::new(), span);
    let closure = build_filter_closure(span);
    let filter_expr = build_method_call(iter_expr, sym::filter, ThinVec::from([closure]), span);
    let cloned_expr = build_method_call(filter_expr, sym::cloned, ThinVec::new(), span);
    let collect_expr = build_method_call(cloned_expr, sym::collect, ThinVec::new(), span);

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

/// Build closure: |x| f(x)
fn build_filter_closure(span: Span) -> Box<ast::Expr> {
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
            kind: ast::PatKind::Ident(ast::BindingMode::NONE, Ident::new(sym::x, span), None),
            span,
            tokens: None,
        }),
        id: ast::DUMMY_NODE_ID,
        span,
        is_placeholder: false,
    };

    Box::new(ast::Expr {
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
    })
}
