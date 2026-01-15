#!/usr/bin/env rustc
// Simple test to verify float type upgrading
use wasp::wasp_parser::WaspParser;
use wasp::wasm_gc_emitter::eval;
use wasp::node::Node;
use wasp::Number;

fn main() {
    // Test 1+π (integer + float = float)
    let result = eval("1+π");
    println!("1+π = {:?}", result);
    
    // assert!()if it's a float with the correct value
    let pi = std::f64::consts::PI;
    match result {
        Node::Number(Number::Float(f)) => {
            println!("Got Float: {}", f);
            assert!((f - (1.0 + pi)).abs() < 0.0001, "Expected {} but got {}", 1.0 + pi, f);
            println!("✓ Type upgrading works correctly!");
        }
        other => panic!("Expected Float, got {:?}", other),
    }
}
