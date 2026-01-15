#!/usr/bin/env rustc
use warp::parse;
fn main() {
    println!("if(zero()){{3}} => {:?}", parse("if(zero()){3}"));
    println!("if two() {{3}} => {:?}", parse("if two() {3}"));
    println!("if two() {{3}} else {{4}} => {:?}", parse("if two() {3} else {4}"));
}
