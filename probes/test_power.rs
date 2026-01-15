#!/usr/bin/env rustc
// Test power operator **

fn main() {
    // Test parsing of **
    let a = 2;
    let b = 3;
    let result = a ** b;
    println!("2 ** 3 = {}", result);
}
