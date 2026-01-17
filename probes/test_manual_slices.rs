#!/usr/bin/env rust
// import "slices"

a := @[1, 2]
b := @[1, 2]
result := slices.Equal(a, b)
put!("Result: %v\n", result)
