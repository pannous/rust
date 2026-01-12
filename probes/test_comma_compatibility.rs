#!/usr/bin/env rustc

// Test all existing syntaxes still work
fmt.Println("=== Testing all syntax variations ===")

// Original syntaxes
z1 := map{"a": 1, "b": 2}
z2 := {a: 1, b: 2}
put!("map{}: %v\n", z1)
put!("{}: %v\n", z2)

// New bracket syntax - with commas (backward compatible)
z3 := map[active: true, age: 30, name: "Alice"]
put!("map[] with commas: %v\n", z3)

// New bracket syntax - without commas (new feature!)
z4 := map[x: 10 y: 20 z: 30]
put!("map[] no commas: %v\n", z4)

// Edge cases
empty := map[]
trailing := map[a: 1, b: 2,]
trailingNoComma := map[a: 1 b: 2]

put!("Empty: %v\n", empty)
put!("Trailing comma: %v\n", trailing)
put!("No trailing comma: %v\n", trailingNoComma)

put!("All types: %T %T %T %T\n", z1, z2, z3, z4)
