#!/usr/bin/env rust

a := @[1, 2]
b := @[1, 2]

// This should trigger the generation of slice equality function
result := a == b
put!("Result: %v\n", result)

// Test very simple case
a1 := @[1]
b1 := @[1]

put!("Single element: %v == %v: %v\n", a1, b1, a1 == b1)

// Test two elements  
c := @[1, 2]
d := @[1, 2]

put!("Two elements: %v == %v: %v\n", c, d, c == d)

s1 := @[1, 2]
s2 := @[1, 2]
put!("s1 == s2:", s1 == s2)

// Test simple cases to debug
put!("Testing slice comparisons:")

// Test empty slices
e := @[]
f := @[]
put!("Empty slices: %v\n", e == f)

// Test nil slices
let g []int
let h []int
put!("Nil slices: %v\n", g == h)