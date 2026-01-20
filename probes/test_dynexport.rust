#!/usr/bin/env rust
// Test for #[dynexport] attribute - stable dynamic library exports

#[dynexport]
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[dynexport]
pub static MAGIC_NUMBER: i32 = 42;

#[dynexport]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    // Test that the functions work correctly
    eq!(add_numbers(2, 3), 5);
    eq!(MAGIC_NUMBER, 42);
    eq!(greet("World"), "Hello, World!");
    println!("All dynexport tests passed!");
}
