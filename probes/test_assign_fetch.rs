#!/usr/bin/env rustc
use warp::wasp_parser::parse;
fn main() {
    let code = "x=fetch https://pannous.com/files/test";
    let node = parse(code);
    println!("Parsed: {:#?}", node);
}
