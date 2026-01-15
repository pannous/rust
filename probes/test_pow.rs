#!/usr/bin/env rustc
use warp::*;
fn main() {
    // Test basic power
    let r1 = eval("3^2").unwrap();
    println!("3^2 = {:?}", r1);
    
    // Test square function
    let r2 = eval("square:=it^2;square 3").unwrap();
    println!("square 3 = {:?}", r2);
    
    // Test complex expression
    let r3 = eval("square:=it^2;1+square(2+3)").unwrap();
    println!("1+square(2+3) = {:?}", r3);
    
    // Test problematic expression
    let r4 = eval("square:=it^2;1+square 2+3").unwrap();
    println!("1+square 2+3 = {:?}", r4);
}
