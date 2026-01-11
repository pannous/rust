use warp::{is, eq};
use warp::wasm_gc_emitter::eval;

fn main() {
    // Test simple fetch (no variable)
    let r1 = eval("fetch https://pannous.com/files/test");
    println!("fetch URL: {:?}", r1.serialize());
    
    // Test fetch with assignment (no second statement)
    let r2 = eval("x=fetch https://pannous.com/files/test");
    println!("x=fetch URL: {:?}", r2.serialize());
    
    // Test the failing case
    let r3 = eval("x=fetch https://pannous.com/files/test;i=7;x");
    println!("x=fetch;i=7;x: {:?}", r3.serialize());
}
