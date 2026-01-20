#!/usr/bin/env rust
// Standalone test with same imports as test_math.rs
use warp::extensions::print;
use warp::is;
use warp::wasm_gc_emitter::eval;

fn main() {
    put!("Testing variable_minus...");
    
    let result = eval("a=-1; b=2; b - a");
    println!("Result node: {:?}", result);
    println!("Result serialize: {}", result.serialize());
    println!("Result value: {:?}", result.value());
    println!("Result == 3: {}", result == 3);
    
    // This is what the is! macro does
    is!("a=-1; b=2; b - a", 3);
    println!("Test passed!");
}
