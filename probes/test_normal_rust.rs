// Normal Rust file (no shebang) - strings should still be &str
fn main() {
    let s: &str = "hello"; // This should work - string literal is &str
    assert_eq!(s.len(), 5);

    // Type check that it's &str, not String
    fn takes_str(_: &str) {}
    takes_str("world");

    println!("Non-script mode strings work correctly!");
}
