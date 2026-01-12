#!/usr/bin/env rustc

a := []int{1, 2}
b := []int{1, 2}

// This should trigger the generation of slice equality function
result := a == b
put!("Result: %v\n", result)


// Test different slices with same content
c := []int{1, 2}
d := []int{1, 2}
put!("Different slices, same content: %v\n", c == d)

// Test empty slices
e := []int{}
f := []int{}
put!("Empty slices: %v\n", e == f)

// Test nil slices
var g []int
var h []int
put!("Nil slices: %v\n", g == h)

put!("All tests completed successfully.\n")
