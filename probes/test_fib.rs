#!/usr/bin/env rust
use warp::wasm_gc_emitter::WasmGcEmitter;
use warp::wasp_parser::WaspParser;
use warp::write_wasm;

fn main() {
    let input = "fib := it < 2 ? it : fib(it-1) + fib(it-2); fib(10)";
    let node = WaspParser::parse(input);
    println!("Parsed: {}", node.serialize());
    
    let mut emitter = WasmGcEmitter::new();
    emitter.emit_for_node(&node);
    let bytes = emitter.finish();
    
    write_wasm("/tmp/test_fib.wasm", &bytes);
    println!("Written to /tmp/test_fib.wasm");
}
