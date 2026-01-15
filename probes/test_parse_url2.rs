#!/usr/bin/env rustc
use warp::wasp_parser::parse;
fn main() {
    let code = "https://example.com/path";
    let node = parse(code);
    println!("Parsed: {:#?}", node);
}
