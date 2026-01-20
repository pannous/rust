#!/usr/bin/env rust
// Test [1,2,3] slice literal syntax
z := @[1, 2, 3]
put!("Slice z: {:?}\n", z)

// Test accessing elements
put!("First element: {}\n", z[0])
put!("Second element: {}\n", z[1])

// Test mixed types
mixed := @["hello", 42, true]
put!("Mixed slice: {:?}\n", mixed)

// Test empty slice (needs explicit type)
let empty : Vec<i32> = @[]
put!("Empty slice: {:?}\n", empty)
