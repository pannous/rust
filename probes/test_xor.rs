#!/usr/bin/env rustc
fn main() {
    // Test xor as ^
    assert!(true xor false);    // true ^ false = true
    assert!(false xor true);    // false ^ true = true
    assert!(!(true xor true));  // true ^ true = false
    assert!(!(false xor false)); // false ^ false = false
    
    // With integers
    let a: u8 = 0b1010;
    let b: u8 = 0b1100;
    eq!(a xor b, 0b0110);  // 10 ^ 12 = 6
    
    // Mix with other operators
    let x = 5;
    let y = 3;
    eq!(x xor y, x ^ y);
    
    println!("All xor tests passed!");
}
