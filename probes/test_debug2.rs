use warp::wasm_gc_emitter::eval;

fn main() {
    // Test with = instead of :=
    let r1 = eval("a=-1; b=2; b - a");
    println!("a=-1; b=2; b - a => {:?}", r1.serialize());
}
