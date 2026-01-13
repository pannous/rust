#!/usr/bin/env rust
// Test put! and eq! macros for script mode

put!(42)
put!("hello world")

let x = 5;
let y = 5;
eq!(x, y)

let s = "test";
eq!(s, "test")
