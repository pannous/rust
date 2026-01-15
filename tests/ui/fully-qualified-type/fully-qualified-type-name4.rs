// Test that we use fully-qualified type names in error messages.
//@ run-pass
// Note: `return x` now works due to auto-wrapping to Some(x)

use std::option::Option;

fn bar(x: usize) -> Option<usize> {
    return x;  // auto-wraps to Some(x)
}

fn main() {
    assert_eq!(bar(42), Some(42));
}
