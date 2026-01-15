#!/usr/bin/env rustc
use warp::wasp_parser::parse;
fn main() {
    let code = "class Person { name: String age: i64 }";
    let node = parse(code);
    println!("Parsed: {:#?}", node);
}
