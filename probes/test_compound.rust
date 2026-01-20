#!/usr/bin/env rust
use warp::wasm_gc_emitter::eval;

fn main() {
    let r1 = eval("x:=10; x += 5; x");
    println!("x:=10; x += 5; x => {:?}", r1.serialize());
    
    let r2 = eval("x:=10; x -= 3; x");
    println!("x:=10; x -= 3; x => {:?}", r2.serialize());
}
