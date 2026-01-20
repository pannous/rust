#!/usr/bin/env rust
use warp::wasm_gc_emitter::eval;

fn main() {
    // Test exact string from test_variable_minus
    let r = eval("a=-1; b=2; b - a");
    println!("Result: {:?}", r);
    println!("Serialized: {:?}", r.serialize());
    println!("Value: {:?}", r.value());
    println!("Is 3? {}", r.value() == &3_i64);
}
