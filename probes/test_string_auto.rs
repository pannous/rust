#!/usr/bin/env rust
// Test that “foo’ automatically becomes String in script mode

// Test 1: Simple string literal type
let name = “Alice”
// let name = "Alice" // still &str!!!
eq!( std::any::type_name_of_val(&name), "alloc::string::String" )
eq!( typeid!(name), "alloc::string::String" )
// eq!( type(name), "alloc::string::String" )

// Test 2: Can call String methods
let greeting = "Hello"
eq!( greeting.len(), 5 )

// Test 3: String concatenation works
let full = "Hello" + " World"
eq!( full, "Hello World" )

put!("All string auto-conversion tests passed!")
