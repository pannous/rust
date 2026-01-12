#!/usr/bin/env rustc
// Test [1,2,3] slice literal syntax
z := [1, 2, 3]
put!("Slice z: %v\n", z)
put!("Type of z: %v\n", typeof(z))

// Test accessing elements
put!("First element:%v\n", z[0])
put!("Second element:%v\n", z[1])

// Test mixed types
mixed := ["hello", 42, true]
put!("Mixed slice:%v\n", mixed)
put!("Type of mixed:%v\n", typeof(mixed))

// Test empty slice (needs explicit type)
empty := []int{}
put!("Empty slice:%v\n", empty)
