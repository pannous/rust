#!/usr/bin/env rustc
use warp::wasm_gc_emitter::eval;
fn main() {
    let result = eval("square := it * it; square(5)");
    println!("Result: {:?}", result);
}
