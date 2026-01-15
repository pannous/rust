// https://github.com/rust-lang/rust/issues/114392
//@ run-pass
// Note: This now compiles due to auto-wrapping of () to Some(())

fn foo() -> Option<()> {
    let x = Some(());
    (x?)  // x? returns (), which auto-wraps to Some(())
}

fn main() {
    assert_eq!(foo(), Some(()));
}
