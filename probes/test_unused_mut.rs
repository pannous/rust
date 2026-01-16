#!/usr/bin/env rust
// Test that unused_mut warning is suppressed in script mode
__let!(mut x = 5);
__let!(mut y = "hello");
put!(x);
put!(y);
// These muts are unnecessary but should not warn in script mode
