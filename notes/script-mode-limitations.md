# Script Mode Features - Global Migration Status

## Summary

Most custom features are **already global** - they work in all Rust code without `-Z script`.
Only a few features must remain script-mode only due to conflicts.

## Already Global Features

These work everywhere, no script mode required:

| Feature | Location | Notes |
|---------|----------|-------|
| Unicode operators `≤ ≥ ≠ …` | `lexer/mod.rs:438-441` | Lexer level |
| Boolean keywords `and, or, not, xor` | `parser/expr.rs:420-428` | Also `¬ ∧ ∨` |
| Optional chaining `?.` and `??` | `parser/expr.rs:945-1509` | |
| `@[...]` list literals | `parser/expr.rs` | Vec or Vec<Val> |
| `@{...}` map literals | `parser/expr.rs` | HashMap |
| `i++` and `i--` | `parser/diagnostics.rs` | Postfix increment/decrement |
| String `%` formatting | `parser/expr.rs` | printf-style |
| String `+` concatenation | `parser/expr.rs:4621` | With non-strings |
| `:=` walrus operator | `parser/item.rs:433` | `let mut` shorthand |
| `class` keyword | `parser/item.rs:416` | Struct synonym (**migrated**) |
| Curly quote strings `"..."` | `lexer/mod.rs:447` | Produces String (**migrated**) |

## Cannot Make Global

These must stay script-mode only due to fundamental conflicts:

### Go-style return types (`fn foo() int` without `->`)
- Location: `compiler/rustc_parse/src/parser/ty.rs:271-285`
- **Problem**: `can_begin_type()` matches identifiers
- **Breaks closures**: `|x| x.method()` - the `x` after closure param looks like a return type
- **Possible future fix**: Track parse context to only allow for `fn`/`def` items, not closures

### `var` keyword (let mut synonym)
- Location: `compiler/rustc_parse/src/parser/stmt.rs:116-142`
- **Problem**: `var` used as variable/parameter name 50+ times in compiler
- Examples: `fn root_var(&self, var: ty::TyVid)` in rustc_infer

### `def` keyword (fn synonym)
- Location: `compiler/rustc_parse/src/parser/item.rs:3021,3152`
- **Problem**: `def` used as module name throughout compiler (`rustc_hir::def`)
- Would break self-hosting

### Arrow functions (`x => expr` and `(x) => expr`)
- Location: `compiler/rustc_parse/src/parser/expr.rs:1578,1610`
- **Problem**: `=>` is used in match arms, causing fundamental ambiguity
- **Simple form `x => expr`**: Conflicts with match guards `_ if x => result`
- **Parenthesized form `(x) => expr`**: Conflicts with match arms `(pattern) => result`
- **No safe fix possible**: Both Rust match arms and arrow functions use `=>`
- Would require deep context tracking to disambiguate

### Type aliases (`int = i64`, `float = f64`, etc.)
- Location: `transformer/` harness injection
- **Problem**: Would conflict with standard Rust types and third-party libraries
- Every crate uses `i32`, `i64`, `f64` etc.

## Script-Mode Only (Harness Injected)

These features inject code at AST level, require script mode infrastructure:

| Feature | Location | Notes |
|---------|----------|-------|
| Type aliases (`int`, `float`, `bool`) | `transformer/` | Conflicts with std types |
| `put!()` macro | `transformer/macros.rs` | Generous printing |
| Truthy trait | `transformer/truthy.rs` | `if 1 { }` support |
| String helpers | `transformer/string.rs` | 100+ convenience methods |
| Val enum | `transformer/val.rs` | Dynamic typing for `@[...]` |
| File-level statements | `parser/item.rs:69` | Mix items and statements |
| Implicit main | Script harness | Wrap code in `fn main()` |

## Migration History

- **2026-01-17**: Migrated `class` keyword to global
- **2026-01-17**: Migrated curly quote strings to global
- **2026-01-17**: Documented why `var`, `def`, Go-style returns, arrow functions cannot be global
