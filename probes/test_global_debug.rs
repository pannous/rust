#!/usr/bin/env rustc
use warp::wasp_parser::parse;
use warp::wasm_gc_emitter::eval;

fn main() {
    let expr = "global x=1+3.14; x+2";
    let node = parse(expr);
    println!("Parsed: {:?}", node);
    
    let result = eval(expr);
    println!("Result: {:?}", result);
    println!("Value: {:?}", result.value());
}
