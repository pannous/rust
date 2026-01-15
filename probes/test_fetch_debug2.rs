#!/usr/bin/env rustc
use warp::wasp_parser::parse;
use warp::wasm_gc_emitter::WasmGcEmitter;
use warp::wasm_gc_reader::read_bytes_with_host;
use warp::node::Node;
fn main() {
    let code = "fetch https://pannous.com/files/test";
    let node = parse(code);
    println!("Parsed node: {:?}", node);
    
    let mut emitter = WasmGcEmitter::new();
    emitter.set_host_imports(true);
    println!("Host imports enabled: true");
    
    emitter.emit_for_node(&node);
    let bytes = emitter.finish();
    println!("Generated {} bytes of WASM", bytes.len());
    
    // Write WASM to file for inspection
    std::fs::write("/tmp/fetch_test.wasm", &bytes).expect("Failed to write WASM");
    println!("Wrote WASM to /tmp/fetch_test.wasm");
    
    // Try to run it
    println!("Calling read_bytes_with_host...");
    match read_bytes_with_host(&bytes) {
        Ok(result) => println!("Result: {:#?}", result),
        Err(e) => println!("Error: {}", e),
    }
}
