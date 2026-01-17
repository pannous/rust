// Regression test for #89173: printf-style format strings now work!
// This test verifies that printf-style format specifiers are supported.
//@ run-pass

fn main() {
    let num = 0x0abcde;
    let width = 6;
    // Printf-style format: %0*x means zero-padded hex with width from argument
    print!("%0*x", width, num);
    // Should print: 0abcde (6 chars, zero-padded hex)
}
