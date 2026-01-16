#!/usr/bin/env rust
use warp::wasp_parser::parse;
use warp::wasm_gc_emitter::{WasmGcEmitter, eval};
use warp::node::Node;
fn main() {
    let code = "fetch https://pannous.com/files/test";
    let node = parse(code);
    println!("Parsed: {:#?}", node);
    
    // Test extract_url_string manually
    if let Node::List(items, _, _) = node.drop_meta() {
        if items.len() == 2 {
            if let Node::Symbol(s) = items[0].drop_meta() {
                println!("First item is Symbol({})", s);
            }
            println!("Second item: {:#?}", items[1]);
        }
    }
    
    // Now try eval
    println!("\nCalling eval...");
    let result = eval(code);
    println!("Result: {:#?}", result);
}
