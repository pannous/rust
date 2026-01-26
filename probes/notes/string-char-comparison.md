# String-Char Comparison Implementation

## Summary
Implemented parser-level coercion to support comparing string literals with char literals using `==` and `!=` operators.

## Implementation Details

### Location
`compiler/rustc_parse/src/parser/expr.rs` - `mk_binary()` function (lines ~4860-4881)

### Transformation
- `"a" == 'a'` → `"a" == 'a'.to_string()`
- `'b' == "c"` → `'b'.to_string() == "c"`

### Pattern
Similar to existing int-float coercion:
1. Check if binary operation is equality (`Eq` or `Ne`)
2. Detect if one operand is string literal and other is char literal
3. Apply `.to_string()` to the char literal using `wrap_in_to_string()` helper

## Working Cases
✅ Direct comparisons with literals: `assert!("a" == 'a')`
✅ Inequality: `assert!("c" != 'd')`
✅ Both directions: `'a' == "a"` and `"a" == 'a'`

## Limitations
❌ Does not work with `eq!()` macro due to macro expansion timing
- The `eq!()` macro uses `assert_eq!()` which takes references before comparison
- References bypass the literal transformation
- Workaround: Use `assert!()` for string-char comparisons

❌ Only works for literals, not runtime values
- `"hello".first() == 'h'` fails because `.first()` returns `String`, not a literal
- Parser transformations only apply to literal expressions

## Related Files
- `compiler/rustc_lexer/src/cursor.rs` - Added `#[allow(dead_code)]` to suppress warning
- `probes/test_string_comparison.rust` - Test suite (moved from todo/)

## Build Time
- Compiler only: ~3-5 minutes
- With standard library: ~3.5 minutes total

## Future Work
To make this work with `eq!()` macro, would need to:
1. Implement `PartialEq<char>` for `&str` at the trait level, OR
2. Modify the `eq!()` macro to not take references for these comparisons, OR
3. Add a type-checking phase transformation in addition to the parser transformation
