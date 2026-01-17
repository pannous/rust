#!/usr/bin/env rust

a := @[1, 2]
b := @[1, 2]

// This should trigger the generation of slice equality function
result := a == b
put!("Result: {}\n", result)

// Test very simple case
a1 := @[1]
b1 := @[1]

put!("Single element: {:?} == {:?}: {}\n", a1, b1, a1 == b1)

// Test two elements  
c := @[1, 2]
d := @[1, 2]

put!("Two elements: {:?} == {:?}: {}\n", c, d, c == d)

s1 := @[1, 2]
s2 := @[1, 2]
put!("s1 == s2: {}", s1 == s2)

// Test simple cases to debug
put!("Testing slice comparisons:")

// Test empty slices
let e: Vec<i32> = vec![]
let f: Vec<i32> = vec![]
put!("Empty slices e,f: {}\n", e == f)

// Test more empty slices
let g: Vec<i32> = vec![]
let h: Vec<i32> = vec![]
put!("Empty slices g,h: {}\n", g == h)