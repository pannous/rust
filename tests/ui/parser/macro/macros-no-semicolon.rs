//@ check-pass
// Test that semicolons are inferred after macro invocations

fn main() {
    assert_eq!(1, 1)
    assert_eq!(3, 3)
    println!("hello");
}
