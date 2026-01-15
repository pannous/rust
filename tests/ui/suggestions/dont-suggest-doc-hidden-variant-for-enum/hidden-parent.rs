//@ run-pass
// Note: This test no longer needs aux-build since the mismatched types error
// is gone due to auto-wrapping of 1i32 to Some(1i32)

fn main() {
    let x: Option<i32> = 1i32;  // auto-wraps to Some(1i32)
    assert_eq!(x, Some(1));
}
