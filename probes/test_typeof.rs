#!/usr/bin/env rust
// Test typeid! macro

let x = 42;
put!(typeid!(x));  // i32
eq!(typeid!(x), "i32");

let s = "hello";
put!(typeid!(s));  // String (script mode auto-converts)
eq!(typeid!(s), "&str");


let s2 = "hello".to_string();
put!(typeid!(s2));  // String (script mode auto-converts)
eq!(typeid!(s2), "alloc::string::String");

let st = “hello”;
put!(typeid!(st));  // String (script mode auto-converts)
eq!(typeid!(st), "alloc::string::String");

let v = vec![1, 2, 3];
put!(typeid!(v));  // Vec<i32>
eq!(typeid!(v), "alloc::vec::Vec<i32>");

let f = 3.14;
put!(typeid!(f));  // f64
eq!(typeid!(f), "f64");


// Test inline expressions
put!(typeid!(100u8));  // u8
eq!(typeid!(100u8), "u8");
put!(typeid!(true));   // bool
eq!(typeid!(true), "bool");
put!(typeid!(&[1, 2, 3])); // slice
eq!(typeid!(&[1, 2, 3]), "&[i32; 3]");// yikes!
// eq!(typeid!(&[1, 2, 3]), "slice"); // todo ^^
