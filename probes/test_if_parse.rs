#!/usr/bin/env rustc
use warp::parse;

fn main() {
    println!("if 0:3 => {:?}", parse("if 0:3"));
    println!("if(2,3,4) => {:?}", parse("if(2,3,4)"));
    println!("if 0:3 else 4 => {:?}", parse("if 0:3 else 4"));
    println!("if{2,3,4} => {:?}", parse("if{2,3,4}"));
}
