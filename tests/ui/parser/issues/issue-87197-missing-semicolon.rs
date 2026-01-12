//@ check-pass
// Test that semicolons are inferred from newlines.
// https://github.com/rust-lang/rust/issues/87197

fn main() {
    let x = 100
    println!("{}", x)
    let y = 200
    println!("{}", y);
}
