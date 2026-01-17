#!/usr/bin/env rust

a := @[1, 2]
b := @[1, 2]

// This should trigger the generation of slice equality function
result := a == b
put!("Result: %v\n", result)


// Test different slices with same content
c := @[1, 2]
d := @[1, 2]
put!("Different slices, same content: %v\n", c == d)

// Test empty slices
e := @[]
f := @[]
put!("Empty slices: %v\n", e == f)

// Test nil slices
let g []int
let h []int
put!("Nil slices: %v\n", g == h)

put!("All tests completed successfully.\n")
