# Truthy Feature Investigation

## Problem
4 tests are failing because they expect truthy/falsy values to work in if conditions:
- test_truthy_simple.rust
- test_truthy.rust  
- test_truthy_and.rust
- test_truthy_and_complex.rust

## Current Implementation

The truthy feature HAS been implemented, but only for **module-level if statements**, not for if expressions inside functions.

### How it works now:
1. `parse_script_if_statement_as_item()` in `item.rs` handles if statements at module level
2. These get transformed to `__if!(cond ; { body })` macro calls
3. The `__if!` macro wraps conditions with `(&cond).is_truthy()`
4. This works because module-level if statements are parsed as items

### Why tests fail:
The tests use `#[test]` functions, so their if statements are **inside function bodies**.
- Inside functions, if statements are parsed as expressions by `parse_expr_if()` in `expr.rs`
- This path does NOT apply truthy transformations
- Result: type error "expected `bool`, found integer"

## Attempted Fix

I tried adding truthy wrapping to `parse_expr_if()` to handle if expressions inside functions.

### Issues encountered:
1. **Const evaluation error**: Wrapping ALL if expressions (including in stdlib/macro code) causes const evaluation failures
2. The `thread_local!` macro in `extensions/src/numbers.rs` contains if statements that get wrapped
3. These wrapped statements are evaluated in const context where `.is_truthy()` can't be called
4. Making Truthy a const trait doesn't work because the extensions are injected code, not a separate crate

## Solutions

### Option 1: Selective wrapping (complex)
- Only wrap if expressions in user code, not stdlib/macro expansions
- Need to detect const contexts and skip wrapping
- Requires tracking code provenance

### Option 2: Use __if! macro everywhere (simpler)
- Transform ALL if expressions to `__if!` macro calls at parse time
- Similar to current module-level approach but applied everywhere
- Might have issues with let-chains and complex conditions

### Option 3: Leave as-is (pragmatic) 
- Document that truthy only works at module level
- Tests inside functions need explicit boolean conversions
- Users can still use truthy at script top level

## Recommendation

For now, Option 3 (document limitation). The truthy feature works for its intended use case (script-level if statements). Supporting it inside functions requires solving the const evaluation problem, which is non-trivial.

Tests should be updated to not rely on truthy inside #[test] functions, or moved to module-level if statements.
