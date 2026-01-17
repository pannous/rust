# Script Mode Features - Global Migration Status

## Successfully Migrated to Global

### `class` keyword (struct synonym)
- Location: `compiler/rustc_parse/src/parser/item.rs:416`
- Safe because parser requires `class <ident>` pattern
- `let class = ...` still works (different parse context)

## Cannot Make Global

### Go-style return types (`fn foo() int` without `->`)
- Location: `compiler/rustc_parse/src/parser/ty.rs:271-285`
- **Problem**: `can_begin_type()` matches identifiers
- **Breaks closures**: `|x| x.method()` - the `x` after closure param looks like a return type
- **Attempted fix**: Excluding keywords (unsafe, pub, fn, etc.) doesn't help - regular identifiers in closure bodies still match
- **Possible future fix**: Track parse context to only allow Go-style returns for `fn`/`def` items, not closures

### `var` keyword (let mut synonym)
- Location: `compiler/rustc_parse/src/parser/stmt.rs:116-142`
- **Problem**: `var` used as variable/parameter name 50+ times in compiler
- Examples: `fn root_var(&self, var: ty::TyVid)` in rustc_infer

### `def` keyword (fn synonym)
- Location: `compiler/rustc_parse/src/parser/item.rs:3021,3152`
- **Problem**: `def` used as module name throughout compiler (`rustc_hir::def`)
- Would break self-hosting

## Script-Mode Only (Harness Injected)

These features inject code at AST level, harder to make global:
- Type aliases (`int = i64`, `float = f64`)
- `put!()` macro
- Truthy trait
- String helpers
- Val enum
