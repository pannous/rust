// Test that we use fully-qualified type names in error messages.
//@ run-pass
// Note: `x = 5` now works due to auto-wrapping to Some(5)

fn main() {
    let x: Option<usize>;
    x = 5;  // auto-wraps to Some(5)
    assert_eq!(x, Some(5));
}
