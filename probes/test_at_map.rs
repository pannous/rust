#!/usr/bin/env rust
// Test @{key: value} map literal syntax
use std::collections::HashMap;

// Basic map
myMap := @{"a": 1, "b": 2, "c": 3}
put!("Map: {:?}\n", myMap)
put!("Map length: {}\n", myMap.len())

// Map with different types
ages := @{"alice": 30, "bob": 25}
put!("Ages: {:?}\n", ages)

// Empty map (needs explicit type)
let empty: HashMap<String, i32> = @{}
put!("Empty map: {:?}\n", empty)

// Trailing comma
trailing := @{"x": 1, "y": 2,}
put!("Trailing comma map: {:?}\n", trailing)
