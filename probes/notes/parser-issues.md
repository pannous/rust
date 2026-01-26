# Parser Issue: Consecutive Let Statements Without Semicolons

## Problem
When multiple `let` or `:=` statements appear consecutively without semicolons, 
the parser's `parse_expr()` becomes too greedy and tries to parse across newlines,
causing parse errors.

## Examples

### Failing Case
```rust
let x = 5
let y = 10  // ERROR: parser tries to parse this as part of previous expression
```

```rust
add5 := |x| x + 5
mult2 := |x| x * 2  // ERROR: same issue
```

### Working Case (with semicolons)
```rust
let x = 5;
let y = 10;  // Works!
```

## Root Cause
The `parse_expr()` function in `compiler/rustc_parse/src/parser/stmt.rs` doesn't
respect newlines as statement terminators when parsing the right-hand side of
assignments. It continues parsing greedily, trying to incorporate the next line
as part of the current expression.

## Impact
- Affects both regular `let` statements and `:=` operator
- Single assignments work fine
- Consecutive assignments require explicit semicolons

## Potential Solutions
1. Make `parse_expr()` aware of newlines and stop at them in statement context
2. Use a different, less greedy expression parser for assignment right-hand sides
3. Implement backtracking to detect when expression parsing has gone too far

## Attempted Fixes

### Approach 1: STOP_AT_NEWLINE Restriction (Incomplete)
Added a new `Restrictions::STOP_AT_NEWLINE` flag and tried to use it in the `:=`
operator's expression parsing. The idea was to check this restriction in
`should_continue_as_assoc_expr()` and stop parsing when a newline is detected.

**Code added:**
- `compiler/rustc_parse/src/parser/mod.rs`: Added `STOP_AT_NEWLINE` restriction
- `compiler/rustc_parse/src/parser/expr.rs`: Added check in `should_continue_as_assoc_expr()`
- `compiler/rustc_parse/src/parser/stmt.rs`: Used restriction in `:=` operator

**Result:** Did not work. The parser still consumes across newlines.

**Possible reasons:**
1. `can_infer_semi_from_newline()` may be returning false in this context
2. The check in `should_continue_as_assoc_expr()` may be bypassed for closures
3. Closure parsing may have special logic that doesn't go through this path
4. The newline detection may not work correctly for all token types

## Status
- Workaround: Use explicit semicolons for consecutive let/:= statements
- Test file: probes/todo/test_lambda.rust now uses semicolons
- WIP code committed for future investigation
