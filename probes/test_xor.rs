#!/usr/bin/env rustc
fn main() {
    let a = 0b1010;
    let b = 0b1100;

    // Test xor keyword
    let result = a xor b;
    assert!(result == 0b0110);

    // Test ^ operator (should be equivalent)
    assert!(a ^ b == a xor b);

    // Test with booleans
    let t = true;
    let f = false;
    assert!(t xor f == true);
    assert!(t xor t == false);
    assert!(f xor f == false);

    println!("All xor tests passed!");
}
