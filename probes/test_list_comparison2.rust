#!/usr/bin/env rust

a := @[1, 2]
b := @[1, 2]

// This should trigger the generation of slice equality function
result := a == b
put!("Result: {}\n", result)


// Test different slices with same content
c := @[1, 2]
d := @[1, 2]
put!("Different slices, same content: {}\n", c == d)

// Test empty slices
let e: Vec<i32> = vec![]
let f: Vec<i32> = vec![]
put!("Empty slices e,f: {}\n", e == f)

// Test more empty slices
let g: Vec<i32> = vec![]
let h: Vec<i32> = vec![]
put!("Empty slices g,h: {}\n", g == h)

put!("All tests completed successfully.\n")
