#!/usr/bin/env rust
// #import "fmt"  not needed 3398e71181
// Test different map syntax forms

// 1. map{"key": value} syntax
z1 := map{"a": 1, "b": 2}
put!("map{} syntax: %v\n", z1)

// 2. {key: value} syntax (symbol keys converted to strings)
z2 := {a: 1, b: 2}
put!("{} syntax: %v\n", z2)

// 3. map[key: value] syntax
z3 := map[active: true, age: 30, name: "Alice"]
put!("map[] syntax: %v\n", z3)

// Test mixed types
z4 := map[count: 42, enabled: true, title: "test"]
put!("mixed types: %v\n", z4)

// Test map operations
z1["a"] = 10
put!("After modification: %v\n", z1)
put!("Access z1[a]: %v\n", z1["a"])

// Test map comparison  
z5 := {a: 1, b: 2}
put!("z2 == z5: %v\n", z2 == z5) // map can only be compared to nil HARD!
// eq!( z2 , z5   TODO: fix this HARD?);

// Test typeof
put!("typeof(z1): %v\n", typeof(z1))
put!("typeof(z2): %v\n", typeof(z2))
put!("typeof(z3): %v\n", typeof(z3))

// Test with put! for comparison
put!("put! z1: %v\n", z1)

// Test empty maps
empty1 := map{}
empty2 := {}
put!("empty map{}: %v\n", empty1)
put!("empty {}: %v\n", empty2)

// Test complex nesting
# nested := map{
# 	user: {name: "John", age: 30},
# 	settings: {theme: "dark", lang: "en"}
# }
# put!("nested: %v\n", nested)
put!("All tests completed successfully.\n")