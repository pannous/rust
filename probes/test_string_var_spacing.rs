#!/usr/bin/env rustc
// Test string variable interpolation with spacing
name := "world"
result := "hello" name "!"
printf("String var result: '%s'\n", result)
assert_eq!( result , "hello world !");