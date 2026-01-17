//! Val enum for dynamic typing in script mode.
//!
//! Generates a `Val` enum that can hold different types of values,
//! enabling dynamic typing similar to Python or JavaScript.

use rustc_ast as ast;
use rustc_span::{Ident, Span, kw, sym};
use thin_vec::ThinVec;

use super::create_allow_attr;

pub(crate) fn build_val_helpers(def_site: Span, call_site: Span) -> ThinVec<Box<ast::Item>> {
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
pub(crate) fn build_simple_ty(span: Span, name: rustc_span::Symbol) -> Box<ast::Ty> {
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
