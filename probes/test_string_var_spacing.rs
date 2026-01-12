#!/usr/bin/env rustc
// Test string variable interpolation with spacing
name := "world"
result := "hello" name "!"
put!("String var result: '%s'\n", result)
eq!( result , "hello world !");