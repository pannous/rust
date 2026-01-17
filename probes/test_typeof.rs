#!/usr/bin/env rust
// Test typeid! macro

let x = 42;
put!(typeid!(x));  // i32

let s = "hello";
put!(typeid!(s));  // String (script mode auto-converts)

let v = vec![1, 2, 3];
put!(typeid!(v));  // Vec<i32>

let f = 3.14;
put!(typeid!(f));  // f64

let owned = "test".to_string();
put!(typeid!(owned));  // String

// Test inline expressions
put!(typeid!(100u8));  // u8
put!(typeid!(true));   // bool
put!(typeid!(&[1, 2, 3])); // slice
