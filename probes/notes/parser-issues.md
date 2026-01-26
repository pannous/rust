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

## Status
- Workaround: Use explicit semicolons for consecutive let/:= statements
- Test file: probes/todo/test_lambda.rust now uses semicolons
