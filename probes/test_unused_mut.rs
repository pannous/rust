#!/usr/bin/env rust
// Test that unused_mut warning is suppressed in script mode
let mut x = 5;
let mut y = "hello";
put!(x);
// These muts are unnecessary but should not warn in script mode
