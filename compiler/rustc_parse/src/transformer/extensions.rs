//! Parse and inject extension library code for script mode.
//!
//! Instead of generating AST programmatically, this module parses actual Rust
//! source code from the extensions library and injects it into scripts.
//!
//! Extension files are loaded individually via include_str!() so changes to
//! any file will trigger recompilation.

use rustc_ast as ast;
use rustc_session::parse::ParseSess;
use rustc_span::{FileName, Span};
use thin_vec::ThinVec;

use crate::parser::{ForceCollect, Parser};
use crate::source_str_to_stream;

/// Extensions source code - embedded at compile time from individual files.
/// Each file is included separately so changes to any file trigger recompilation.
const TRUTHY_SOURCE: &str = include_str!("../../../extensions/src/truthy.rs");
const STRINGS_SOURCE: &str = include_str!("../../../extensions/src/strings.rs");
const LISTS_SOURCE: &str = include_str!("../../../extensions/src/lists.rs");
const VAL_SOURCE: &str = include_str!("../../../extensions/src/val.rs");
const NUMBERS_SOURCE: &str = include_str!("../../../extensions/src/numbers.rs");
const MACROS_SOURCE: &str = include_str!("../../../extensions/src/macros.rs");

/// Parse and return the extensions items with proper span context.
///
/// Uses `call_site` span so all items are visible to user code.
/// Macros are NOT included here - they need special hygiene handling
/// and should continue to be generated via transformer/macros.rs.
pub fn parse_extensions(
    psess: &ParseSess,
    call_site: Span,
) -> ThinVec<Box<ast::Item>> {
    // Concatenate all extension source files
    let combined_source = [
        TRUTHY_SOURCE,
        STRINGS_SOURCE,
        LISTS_SOURCE,
        VAL_SOURCE,
        NUMBERS_SOURCE,
        MACROS_SOURCE,
    ].join("\n");

    // Extract external crate dependencies from extensions
    let external_crates = extract_external_crates(&combined_source);

    let source_without_macros = filter_out_macros(&combined_source);

    let filename = FileName::Custom("script_extensions".into());

    // Parse with call_site as override_span - this makes all tokens visible
    // to user code by giving them call_site hygiene context
    let stream = match source_str_to_stream(
        psess,
        filename,
        source_without_macros.to_string(),
        Some(call_site),  // Override all spans to call_site for visibility
    ) {
        Ok(stream) => stream,
        Err(errs) => {
            for err in errs {
                err.emit();
            }
            return ThinVec::new();
        }
    };

    // Create parser from the token stream
    let mut parser = Parser::new(psess, stream, None);

    // Parse all items
    let mut items = ThinVec::new();

    // First, inject extern crate declarations for external dependencies
    for crate_name in external_crates {
        let extern_crate = create_extern_crate_item(crate_name, call_site);
        items.push(extern_crate);
    }

    loop {
        match parser.parse_item(ForceCollect::No) {
            Ok(Some(item)) => {
                items.push(item);
            }
            Ok(None) => break,
            Err(err) => {
                err.emit();
                break;
            }
        }
    }

    items
}

/// Extract external crate names from use statements in extension source.
/// Returns a list of crate names that are not std/core/alloc.
fn extract_external_crates(source: &str) -> Vec<&str> {
    let mut crates = Vec::new();
    let stdlib_crates = ["std", "core", "alloc"];

    for line in source.lines() {
        let trimmed = line.trim();
        // Match: use crate_name::...;
        if trimmed.starts_with("use ") {
            if let Some(rest) = trimmed.strip_prefix("use ") {
                // Extract the crate name (first segment before ::)
                if let Some(crate_part) = rest.split("::").next() {
                    let crate_name = crate_part.trim();
                    // Skip stdlib crates and crate/self/super
                    if !stdlib_crates.contains(&crate_name)
                        && crate_name != "crate"
                        && crate_name != "self"
                        && crate_name != "super"
                        && !crates.contains(&crate_name)
                    {
                        crates.push(crate_name);
                    }
                }
            }
        }
    }

    crates
}

/// Create an extern crate item: extern crate name;
fn create_extern_crate_item(crate_name: &str, span: Span) -> Box<ast::Item> {
    use rustc_span::{Ident, Symbol};

    let ident = Ident::new(Symbol::intern(crate_name), span);

    Box::new(ast::Item {
        attrs: thin_vec::ThinVec::new(),
        id: ast::DUMMY_NODE_ID,
        kind: ast::ItemKind::ExternCrate(None, ident),
        vis: ast::Visibility {
            span,
            kind: ast::VisibilityKind::Inherited,
            tokens: None,
        },
        span,
        tokens: None,
    })
}

/// Filter out macro definitions from source code.
/// Macros need special hygiene handling so we generate them separately.
fn filter_out_macros(source: &str) -> String {
    let mut result = String::new();
    let mut lines = source.lines().peekable();
    let mut in_macro = false;
    let mut brace_depth = 0;

    while let Some(line) = lines.next() {
        let trimmed = line.trim();

        // Check if next line is a macro - if so, skip any attributes
        if let Some(next_line) = lines.peek() {
            if next_line.contains("macro_rules!") || next_line.trim().starts_with("#[macro_export]") {
                // Current line is an attribute before a macro - skip it
                if trimmed.starts_with("#[") {
                    continue;
                }
            }
        }

        // Check for #[macro_export] - skip this line and the following macro
        if trimmed.starts_with("#[macro_export]") {
            in_macro = true;
            brace_depth = 0;
            continue;
        }

        // Check for macro_rules! start
        if line.contains("macro_rules!") {
            in_macro = true;
            brace_depth = 0;
            // Count braces in this line
            for c in line.chars() {
                if c == '{' {
                    brace_depth += 1;
                } else if c == '}' {
                    brace_depth -= 1;
                }
            }
            // Check if macro closed on same line
            if brace_depth == 0 && line.contains('{') {
                in_macro = false;
            }
            continue;
        }

        if in_macro {
            // Count braces to find end of macro
            for c in line.chars() {
                if c == '{' {
                    brace_depth += 1;
                } else if c == '}' {
                    brace_depth -= 1;
                    if brace_depth == 0 {
                        in_macro = false;
                        break;
                    }
                }
            }
            continue;
        }

        // Not in a macro - keep this line
        result.push_str(line);
        result.push('\n');
    }

    result
}
