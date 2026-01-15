#!/usr/bin/env rustc
use warp::wasp_parser::parse;
fn main() {
    // Test just the body without class wrapper
    let code = "{name:String age:i64}";
    let node = parse(code);
    println!("Body only: {:#?}", node);
    
    // Now test full class
    let code2 = "class Person{name:String age:i64}";
    let node2 = parse(code2);
    println!("\nFull class: {:#?}", node2);
}
