use Piece::*;

use super::*;

#[track_caller]
fn same(fmt: &'static str, p: &[Piece<'static>]) {
    let parser = Parser::new(fmt, None, None, false, ParseMode::Format);
    assert_eq!(parser.collect::<Vec<Piece<'static>>>(), p);
}

fn fmtdflt() -> FormatSpec<'static> {
    return FormatSpec {
        fill: None,
        fill_span: None,
        align: AlignUnknown,
        sign: None,
        alternate: false,
        zero_pad: false,
        debug_hex: None,
        precision: CountImplied,
        width: CountImplied,
        precision_span: None,
        width_span: None,
        ty: "",
        ty_span: None,
    };
}

fn musterr(s: &str) {
    let mut p = Parser::new(s, None, None, false, ParseMode::Format);
    p.next();
    assert!(!p.errors.is_empty());
}

#[test]
fn simple() {
    same("asdf", &[Lit("asdf")]);
    same("a{{b", &[Lit("a"), Lit("{b")]);
    same("a}}b", &[Lit("a"), Lit("}b")]);
    same("a}}", &[Lit("a"), Lit("}")]);
    same("}}", &[Lit("}")]);
    same("\\}}", &[Lit("\\"), Lit("}")]);
}
#[test]
fn invalid01() {
    musterr("{")
}
#[test]
fn invalid02() {
    musterr("}")
}
#[test]
fn invalid04() {
    musterr("{3a}")
}
#[test]
fn invalid05() {
    musterr("{:|}")
}
#[test]
fn invalid06() {
    musterr("{:>>>}")
}

#[test]
fn invalid_position() {
    musterr("{18446744073709551616}");
}

#[test]
fn invalid_width() {
    musterr("{:18446744073709551616}");
}

#[test]
fn invalid_precision() {
    musterr("{:.18446744073709551616}");
}

#[test]
fn format_empty() {
    same(
        "{}",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 2..2,
            format: fmtdflt(),
        }))],
    );
}
#[test]
fn format_tab_empty() {
    let fmt_pre = r###""\t{}""###;
    let fmt = "\t{}";
    let parser = Parser::new(fmt, None, Some(fmt_pre.into()), false, ParseMode::Format);
    assert_eq!(
        parser.collect::<Vec<Piece<'static>>>(),
        &[
            Lit("\t"),
            NextArgument(Box::new(Argument {
                position: ArgumentImplicitlyIs(0),
                position_span: 4..4,
                format: fmtdflt(),
            }))
        ],
    );
}
#[test]
fn format_open_brace_tab() {
    let fmt_pre = r###""{\t""###;
    let fmt = "{\t";
    let mut parser = Parser::new(fmt, None, Some(fmt_pre.into()), false, ParseMode::Format);
    let _ = parser.by_ref().collect::<Vec<Piece<'static>>>();
    assert_eq!(parser.errors[0].span, 4..4);
}
#[test]
fn format_position() {
    same(
        "{3}",
        &[NextArgument(Box::new(Argument {
            position: ArgumentIs(3),
            position_span: 2..3,
            format: fmtdflt(),
        }))],
    );
}
#[test]
fn format_position_nothing_else() {
    same(
        "{3:}",
        &[NextArgument(Box::new(Argument {
            position: ArgumentIs(3),
            position_span: 2..3,
            format: fmtdflt(),
        }))],
    );
}
#[test]
fn format_named() {
    same(
        "{name}",
        &[NextArgument(Box::new(Argument {
            position: ArgumentNamed("name"),
            position_span: 2..6,
            format: fmtdflt(),
        }))],
    )
}
#[test]
fn format_named_space_nothing() {
    same(
        "{name} {}",
        &[
            NextArgument(Box::new(Argument {
                position: ArgumentNamed("name"),
                position_span: 2..6,
                format: fmtdflt(),
            })),
            Lit(" "),
            NextArgument(Box::new(Argument {
                position: ArgumentImplicitlyIs(0),
                position_span: 9..9,
                format: fmtdflt(),
            })),
        ],
    )
}
#[test]
fn format_raw() {
    let snippet = r###"r#"assertion `left {op} right` failed"#"###.into();
    let source = r#"assertion `left {op} right` failed"#;

    let parser = Parser::new(source, Some(1), Some(snippet), true, ParseMode::Format);
    let expected = &[
        Lit("assertion `left "),
        NextArgument(Box::new(Argument {
            position: ArgumentNamed("op"),
            position_span: 20..22,
            format: fmtdflt(),
        })),
        Lit(" right` failed"),
    ];
    assert_eq!(parser.collect::<Vec<Piece<'static>>>(), expected);
}
#[test]
fn format_type() {
    same(
        "{3:x}",
        &[NextArgument(Box::new(Argument {
            position: ArgumentIs(3),
            position_span: 2..3,
            format: FormatSpec {
                fill: None,
                fill_span: None,
                align: AlignUnknown,
                sign: None,
                alternate: false,
                zero_pad: false,
                debug_hex: None,
                precision: CountImplied,
                width: CountImplied,
                precision_span: None,
                width_span: None,
                ty: "x",
                ty_span: None,
            },
        }))],
    );
}
#[test]
fn format_align_fill() {
    same(
        "{3:>}",
        &[NextArgument(Box::new(Argument {
            position: ArgumentIs(3),
            position_span: 2..3,
            format: FormatSpec {
                fill: None,
                fill_span: None,
                align: AlignRight,
                sign: None,
                alternate: false,
                zero_pad: false,
                debug_hex: None,
                precision: CountImplied,
                width: CountImplied,
                precision_span: None,
                width_span: None,
                ty: "",
                ty_span: None,
            },
        }))],
    );
    same(
        "{3:0<}",
        &[NextArgument(Box::new(Argument {
            position: ArgumentIs(3),
            position_span: 2..3,
            format: FormatSpec {
                fill: Some('0'),
                fill_span: Some(4..5),
                align: AlignLeft,
                sign: None,
                alternate: false,
                zero_pad: false,
                debug_hex: None,
                precision: CountImplied,
                width: CountImplied,
                precision_span: None,
                width_span: None,
                ty: "",
                ty_span: None,
            },
        }))],
    );
    same(
        "{3:*<abcd}",
        &[NextArgument(Box::new(Argument {
            position: ArgumentIs(3),
            position_span: 2..3,
            format: FormatSpec {
                fill: Some('*'),
                fill_span: Some(4..5),
                align: AlignLeft,
                sign: None,
                alternate: false,
                zero_pad: false,
                debug_hex: None,
                precision: CountImplied,
                width: CountImplied,
                precision_span: None,
                width_span: None,
                ty: "abcd",
                ty_span: Some(6..10),
            },
        }))],
    );
}
#[test]
fn format_counts() {
    same(
        "{:10x}",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 2..2,
            format: FormatSpec {
                fill: None,
                fill_span: None,
                align: AlignUnknown,
                sign: None,
                alternate: false,
                zero_pad: false,
                debug_hex: None,
                precision: CountImplied,
                precision_span: None,
                width: CountIs(10),
                width_span: Some(3..5),
                ty: "x",
                ty_span: None,
            },
        }))],
    );
    same(
        "{:10$.10x}",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 2..2,
            format: FormatSpec {
                fill: None,
                fill_span: None,
                align: AlignUnknown,
                sign: None,
                alternate: false,
                zero_pad: false,
                debug_hex: None,
                precision: CountIs(10),
                precision_span: Some(6..9),
                width: CountIsParam(10),
                width_span: Some(3..6),
                ty: "x",
                ty_span: None,
            },
        }))],
    );
    same(
        "{1:0$.10x}",
        &[NextArgument(Box::new(Argument {
            position: ArgumentIs(1),
            position_span: 2..3,
            format: FormatSpec {
                fill: None,
                fill_span: None,
                align: AlignUnknown,
                sign: None,
                alternate: false,
                zero_pad: false,
                debug_hex: None,
                precision: CountIs(10),
                precision_span: Some(6..9),
                width: CountIsParam(0),
                width_span: Some(4..6),
                ty: "x",
                ty_span: None,
            },
        }))],
    );
    same(
        "{:.*x}",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(1),
            position_span: 2..2,
            format: FormatSpec {
                fill: None,
                fill_span: None,
                align: AlignUnknown,
                sign: None,
                alternate: false,
                zero_pad: false,
                debug_hex: None,
                precision: CountIsStar(0),
                precision_span: Some(3..5),
                width: CountImplied,
                width_span: None,
                ty: "x",
                ty_span: None,
            },
        }))],
    );
    same(
        "{:.10$x}",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 2..2,
            format: FormatSpec {
                fill: None,
                fill_span: None,
                align: AlignUnknown,
                sign: None,
                alternate: false,
                zero_pad: false,
                debug_hex: None,
                precision: CountIsParam(10),
                width: CountImplied,
                precision_span: Some(3..7),
                width_span: None,
                ty: "x",
                ty_span: None,
            },
        }))],
    );
    same(
        "{:a$.b$?}",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 2..2,
            format: FormatSpec {
                fill: None,
                fill_span: None,
                align: AlignUnknown,
                sign: None,
                alternate: false,
                zero_pad: false,
                debug_hex: None,
                precision: CountIsName("b", 6..7),
                precision_span: Some(5..8),
                width: CountIsName("a", 3..4),
                width_span: Some(3..5),
                ty: "?",
                ty_span: None,
            },
        }))],
    );
    same(
        "{:.4}",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 2..2,
            format: FormatSpec {
                fill: None,
                fill_span: None,
                align: AlignUnknown,
                sign: None,
                alternate: false,
                zero_pad: false,
                debug_hex: None,
                precision: CountIs(4),
                precision_span: Some(3..5),
                width: CountImplied,
                width_span: None,
                ty: "",
                ty_span: None,
            },
        }))],
    )
}
#[test]
fn format_flags() {
    same(
        "{:-}",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 2..2,
            format: FormatSpec {
                fill: None,
                fill_span: None,
                align: AlignUnknown,
                sign: Some(Sign::Minus),
                alternate: false,
                zero_pad: false,
                debug_hex: None,
                precision: CountImplied,
                width: CountImplied,
                precision_span: None,
                width_span: None,
                ty: "",
                ty_span: None,
            },
        }))],
    );
    same(
        "{:+#}",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 2..2,
            format: FormatSpec {
                fill: None,
                fill_span: None,
                align: AlignUnknown,
                sign: Some(Sign::Plus),
                alternate: true,
                zero_pad: false,
                debug_hex: None,
                precision: CountImplied,
                width: CountImplied,
                precision_span: None,
                width_span: None,
                ty: "",
                ty_span: None,
            },
        }))],
    );
}
#[test]
fn format_mixture() {
    same(
        "abcd {3:x} efg",
        &[
            Lit("abcd "),
            NextArgument(Box::new(Argument {
                position: ArgumentIs(3),
                position_span: 7..8,
                format: FormatSpec {
                    fill: None,
                    fill_span: None,
                    align: AlignUnknown,
                    sign: None,
                    alternate: false,
                    zero_pad: false,
                    debug_hex: None,
                    precision: CountImplied,
                    width: CountImplied,
                    precision_span: None,
                    width_span: None,
                    ty: "x",
                    ty_span: None,
                },
            })),
            Lit(" efg"),
        ],
    );
}
#[test]
fn format_whitespace() {
    same(
        "{ }",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 2..3,
            format: fmtdflt(),
        }))],
    );
    same(
        "{  }",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 2..4,
            format: fmtdflt(),
        }))],
    );
}
#[test]
fn asm_linespans() {
    let asm_pre = r###"r"
        .intel_syntax noprefix
        nop""###;
    let asm = r"
        .intel_syntax noprefix
        nop";
    let mut parser = Parser::new(asm, Some(0), Some(asm_pre.into()), false, ParseMode::InlineAsm);
    assert!(parser.is_source_literal);
    assert_eq!(
        parser.by_ref().collect::<Vec<Piece<'static>>>(),
        &[Lit("\n        .intel_syntax noprefix\n        nop")]
    );
    assert_eq!(parser.line_spans, &[2..2, 11..33, 42..45]);
}
#[test]
fn asm_concat() {
    let asm_pre = r###"concat!("invalid", "_", "instruction")"###;
    let asm = "invalid_instruction";
    let mut parser = Parser::new(asm, None, Some(asm_pre.into()), false, ParseMode::InlineAsm);
    assert!(!parser.is_source_literal);
    assert_eq!(parser.by_ref().collect::<Vec<Piece<'static>>>(), &[Lit(asm)]);
    assert_eq!(parser.line_spans, &[]);
}

#[test]
fn diagnostic_format_flags() {
    let lit = "{thing:blah}";
    let mut parser = Parser::new(lit, None, None, false, ParseMode::Diagnostic);
    assert!(!parser.is_source_literal);

    let [NextArgument(arg)] = &*parser.by_ref().collect::<Vec<Piece<'static>>>() else { panic!() };

    assert_eq!(
        **arg,
        Argument {
            position: ArgumentNamed("thing"),
            position_span: 2..7,
            format: FormatSpec { ty: ":blah", ty_span: Some(7..12), ..Default::default() },
        }
    );

    assert_eq!(parser.line_spans, &[]);
    assert!(parser.errors.is_empty());
}

#[test]
fn diagnostic_format_mod() {
    let lit = "{thing:+}";
    let mut parser = Parser::new(lit, None, None, false, ParseMode::Diagnostic);
    assert!(!parser.is_source_literal);

    let [NextArgument(arg)] = &*parser.by_ref().collect::<Vec<Piece<'static>>>() else { panic!() };

    assert_eq!(
        **arg,
        Argument {
            position: ArgumentNamed("thing"),
            position_span: 2..7,
            format: FormatSpec { ty: ":+", ty_span: Some(7..9), ..Default::default() },
        }
    );

    assert_eq!(parser.line_spans, &[]);
    assert!(parser.errors.is_empty());
}

// ============================================================================
// Printf-style format specifier tests
// ============================================================================

#[test]
fn printf_basic_specifiers() {
    // %d - integer (Display)
    same(
        "%d",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 1..3,
            format: fmtdflt(),
        }))],
    );

    // %s - string (Display)
    same(
        "%s",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 1..3,
            format: fmtdflt(),
        }))],
    );

    // %x - lowercase hex
    same(
        "%x",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 1..3,
            format: FormatSpec {
                ty: "x",
                ..fmtdflt()
            },
        }))],
    );

    // %X - uppercase hex
    same(
        "%X",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 1..3,
            format: FormatSpec {
                ty: "X",
                ..fmtdflt()
            },
        }))],
    );

    // %o - octal
    same(
        "%o",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 1..3,
            format: FormatSpec {
                ty: "o",
                ..fmtdflt()
            },
        }))],
    );

    // %e - lowercase exponential
    same(
        "%e",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 1..3,
            format: FormatSpec {
                ty: "e",
                ..fmtdflt()
            },
        }))],
    );

    // %E - uppercase exponential
    same(
        "%E",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 1..3,
            format: FormatSpec {
                ty: "E",
                ..fmtdflt()
            },
        }))],
    );

    // %b - binary (extension)
    same(
        "%b",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 1..3,
            format: FormatSpec {
                ty: "b",
                ..fmtdflt()
            },
        }))],
    );

    // %p - pointer
    same(
        "%p",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 1..3,
            format: FormatSpec {
                ty: "p",
                ..fmtdflt()
            },
        }))],
    );

    // %? - debug (extension)
    same(
        "%?",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 1..3,
            format: FormatSpec {
                ty: "?",
                ..fmtdflt()
            },
        }))],
    );
}

#[test]
fn printf_width() {
    // %10d - width of 10
    same(
        "%10d",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 1..5,
            format: FormatSpec {
                width: CountIs(10),
                ..fmtdflt()
            },
        }))],
    );
}

#[test]
fn printf_precision() {
    // %.2f - precision of 2
    same(
        "%.2f",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 1..5,
            format: FormatSpec {
                precision: CountIs(2),
                ..fmtdflt()
            },
        }))],
    );
}

#[test]
fn printf_width_and_precision() {
    // %10.2f - width 10, precision 2
    same(
        "%10.2f",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 1..7,
            format: FormatSpec {
                width: CountIs(10),
                precision: CountIs(2),
                ..fmtdflt()
            },
        }))],
    );
}

#[test]
fn printf_flags() {
    // %-d - left align
    same(
        "%-d",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 1..4,
            format: FormatSpec {
                align: AlignLeft,
                ..fmtdflt()
            },
        }))],
    );

    // %+d - show sign
    same(
        "%+d",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 1..4,
            format: FormatSpec {
                sign: Some(Sign::Plus),
                ..fmtdflt()
            },
        }))],
    );

    // %#x - alternate form
    same(
        "%#x",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 1..4,
            format: FormatSpec {
                alternate: true,
                ty: "x",
                ..fmtdflt()
            },
        }))],
    );

    // %05d - zero padding
    same(
        "%05d",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 1..5,
            format: FormatSpec {
                zero_pad: true,
                width: CountIs(5),
                ..fmtdflt()
            },
        }))],
    );
}

#[test]
fn printf_combined_flags() {
    // %-+#010.5x - all flags combined
    same(
        "%-+#010.5x",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 1..11,
            format: FormatSpec {
                align: AlignLeft,
                sign: Some(Sign::Plus),
                alternate: true,
                zero_pad: true,
                width: CountIs(10),
                precision: CountIs(5),
                ty: "x",
                ..fmtdflt()
            },
        }))],
    );
}

#[test]
fn printf_positional() {
    // %1$d - first argument (0-indexed in Rust)
    same(
        "%1$d",
        &[NextArgument(Box::new(Argument {
            position: ArgumentIs(0),
            position_span: 1..5,
            format: fmtdflt(),
        }))],
    );

    // %2$d - second argument
    same(
        "%2$d",
        &[NextArgument(Box::new(Argument {
            position: ArgumentIs(1),
            position_span: 1..5,
            format: fmtdflt(),
        }))],
    );
}

#[test]
fn printf_escape() {
    // %% - literal percent
    same("%%", &[Lit("%")]);

    // Text with %% in the middle
    same("50%% off", &[Lit("50"), Lit("% off")]);
}

#[test]
fn printf_star_width() {
    // %*d - width from argument
    same(
        "%*d",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(1),
            position_span: 1..4,
            format: FormatSpec {
                width: CountIsStar(0),
                ..fmtdflt()
            },
        }))],
    );
}

#[test]
fn printf_star_precision() {
    // %.*f - precision from argument
    same(
        "%.*f",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(1),
            position_span: 1..5,
            format: FormatSpec {
                precision: CountIsStar(0),
                ..fmtdflt()
            },
        }))],
    );
}

#[test]
fn printf_length_modifiers() {
    // Length modifiers should be parsed but ignored
    // %ld - long int
    same(
        "%ld",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 1..4,
            format: fmtdflt(),
        }))],
    );

    // %lld - long long int
    same(
        "%lld",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 1..5,
            format: fmtdflt(),
        }))],
    );

    // %hd - short int
    same(
        "%hd",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 1..4,
            format: fmtdflt(),
        }))],
    );

    // %hhd - char
    same(
        "%hhd",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 1..5,
            format: fmtdflt(),
        }))],
    );

    // %zd - size_t
    same(
        "%zd",
        &[NextArgument(Box::new(Argument {
            position: ArgumentImplicitlyIs(0),
            position_span: 1..4,
            format: fmtdflt(),
        }))],
    );
}

#[test]
fn printf_mixed_with_rust() {
    // Mix {} and %d in the same string
    // Note: {} uses Rust-style span (inside braces), %d uses printf-style span (whole specifier)
    same(
        "{} and %d",
        &[
            NextArgument(Box::new(Argument {
                position: ArgumentImplicitlyIs(0),
                position_span: 2..2, // Rust-style: position inside braces (empty)
                format: fmtdflt(),
            })),
            Lit(" and "),
            NextArgument(Box::new(Argument {
                position: ArgumentImplicitlyIs(1),
                position_span: 8..10, // Printf-style: includes %d
                format: fmtdflt(),
            })),
        ],
    );
}

#[test]
fn printf_multiple_args() {
    // Multiple printf specifiers
    same(
        "%d %s %x",
        &[
            NextArgument(Box::new(Argument {
                position: ArgumentImplicitlyIs(0),
                position_span: 1..3,
                format: fmtdflt(),
            })),
            Lit(" "),
            NextArgument(Box::new(Argument {
                position: ArgumentImplicitlyIs(1),
                position_span: 4..6,
                format: fmtdflt(),
            })),
            Lit(" "),
            NextArgument(Box::new(Argument {
                position: ArgumentImplicitlyIs(2),
                position_span: 7..9,
                format: FormatSpec {
                    ty: "x",
                    ..fmtdflt()
                },
            })),
        ],
    );
}

#[test]
fn printf_with_text() {
    // Printf specifier with surrounding text
    same(
        "Value: %d end",
        &[
            Lit("Value: "),
            NextArgument(Box::new(Argument {
                position: ArgumentImplicitlyIs(0),
                position_span: 8..10,
                format: fmtdflt(),
            })),
            Lit(" end"),
        ],
    );
}

#[test]
fn printf_invalid_specifier_is_literal() {
    // Invalid printf specifier should be treated as literal
    // %q is not a valid specifier - becomes two literals due to string() stopping at %
    same("%q", &[Lit("%q")]);

    // Just % at end of string - splits into two literals
    same("test%", &[Lit("test"), Lit("%")]);
}
