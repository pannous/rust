# := Operator Fix

## Problem
The `:=` operator was defined in README but not fully implemented.
Error: `expected one of '!', '.', '::', ';', '?', '{', or an operator, found ':='`

## Root Cause
The lexer generates separate `Colon` and `Eq` tokens, not a single `ColonEq` token.
The parser was checking for `token::ColonEq` which doesn't exist in the token stream.

## Solution
Modified `/opt/other/rust/compiler/rustc_parse/src/parser/stmt.rs`:
1. Check for `token::Colon` followed by lookahead for `token::Eq`
2. Change binding mode from `NONE` to `MUT` for mutable variables

## Changes
```rust
// Line 228: Check for colon followed by equals
if this.token == token::Colon && this.look_ahead(1, |t| *t == token::Eq) {
    // Line 235: Make variable mutable
    let pat = Box::new(this.mk_pat_ident(lo, ast::BindingMode::MUT, ident));
}
```

## Status
Code changes complete, rebuild in progress.
