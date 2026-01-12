When loading functions via dlsym that return structs, you must use extern "C" fn type, not plain fn, otherwise struct return values are corrupted.

## Adding C++ style `and`/`or` operators (2026-01-11)

### Key insight: The parser already had the logic!

The Rust parser (`compiler/rustc_parse/src/parser/expr.rs`) already recognized `and` and `or` as logical operators - but treated them as **errors with recovery**. The recovery code would:
1. Emit an error diagnostic
2. Still parse them as `&&`/`||` to continue compilation

To enable them as valid operators, I just removed the error emission (lines 396-400 and 404-408 in `check_assoc_op()`).

### Files involved:
- `compiler/rustc_parse/src/parser/expr.rs` - Main parser, `check_assoc_op()` function
- `compiler/rustc_parse/src/errors.rs` - Error types (removed dead code after change)
- `compiler/rustc_ast/src/util/parser.rs` - `AssocOp::from_token()` maps tokens to operators
- `compiler/rustc_ast/src/token.rs` - Token definitions (`AndAnd`, `OrOr`, etc.)

### Token → Operator flow:
1. Lexer produces `token::AndAnd` for `&&`
2. `AssocOp::from_token()` maps it to `AssocOp::Binary(BinOpKind::And)`
3. For `and`/`or` identifiers, `check_assoc_op()` checks `self.token.ident()` and maps directly

### Symbols:
The `sym::and` and `sym::or` symbols are already defined in rustc_span. No need to add new keywords.

### Build notes:
- `RUSTC_WRAPPER=""` needed when sccache has issues with incremental compilation
- Stage 1 compiler build: ~15-25 seconds
- Stage 1 library build: ~20 seconds (needed to actually run test programs)
- Test with: `./build/host/stage1/bin/rustc test.rs -o test && ./test`

### Dead code cleanup:
After removing error emission, had to remove unused error types to pass `-Dwarnings`:
- `InvalidLogicalOperator` struct
- `InvalidLogicalOperatorSub` enum

## Adding `not` and `¬` as aliases for `!` (2026-01-11)

### Key insight: Recovery code pattern again!

Similar to `and`/`or`, the parser already had recovery code for `not`:
- `is_mistaken_not_ident_negation()` detected `not` used as negation
- `recover_not_expr()` emitted an error but still parsed it

### The tricky part: Detecting `not` as operator vs identifier

The original code was conservative - only matching `not` followed by:
- Identifiers that can begin expressions
- Literals
- `#` (attributes)

This **excluded** `(` (parentheses), so `not (x < 0)` didn't work!

Fix: Use `t.can_begin_expr()` which includes `(`, `{`, `[`, etc.
But exclude `.` to avoid `not.method()` being misinterpreted.

### Adding `¬` to the lexer

The lexer (`compiler/rustc_lexer/src/lib.rs`) has a simple match:
```rust
'!' | '¬' => Bang,  // ¬ is U+00AC NOT SIGN
```

This is all that's needed - the lexer produces `Bang` token for both.

## Adding Unicode comparison operators (2026-01-11)

### Implementation approach:

Unicode operators are handled in the higher-level lexer at `compiler/rustc_parse/src/lexer/mod.rs`. When an unknown character is encountered:

1. Check if it's one of our supported Unicode operators
2. If so, return the corresponding token directly without emitting an error
3. Otherwise, fall through to the existing error handling

### Characters implemented:
- `≤` (U+2264) → `token::Le` (less than or equal)
- `≥` (U+2265) → `token::Ge` (greater than or equal)  
- `≠` (U+2260) → `token::Ne` (not equal)
- `…` (U+2026) → `token::DotDot` (range syntax)

### Key code change in lexer/mod.rs:
```rust
// Unicode comparison operators - silently accept as valid syntax
if let Some(tok) = match c {
    '≤' => Some(token::Le),
    '≥' => Some(token::Ge),
    '≠' => Some(token::Ne),
    '…' => Some(token::DotDot),
    _ => None,
} {
    tok
} else {
    // ... existing error handling
}
```

### Test:
`probes/test_unicode_ops.rs` - Tests all Unicode operators

## Power operator ** (partial implementation)

### What works:
- Parser accepts `**` syntax
- Added `BinOpKind::Pow` to AST
- Precedence set higher than multiplication
- Right-associative (2**3**4 = 2**(3**4))

### What doesn't work:
- MIR layer doesn't have `BinOp::Pow`
- Would need to add throughout: rustc_middle/mir, rustc_mir_build, rustc_codegen_*
- Currently ICEs when trying to compile code with `**`

### Parser detection of `**`:
In `check_assoc_op()`, when we see `Star` token, look ahead for another `Star`:
```rust
(Some(AssocOp::Binary(BinOpKind::Mul)), _)
    if self.look_ahead(1, |t| t.kind == token::Star) =>
{
    (AssocOp::Binary(BinOpKind::Pow), self.token.span.to(...))
}
```
