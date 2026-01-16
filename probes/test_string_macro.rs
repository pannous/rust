#!/usr/bin/env rustx

// Test s! macro for string literal to String conversion
x := s!("hello")
eq!(x, String::from("hello"))

// Can be used inline
eq!(s!("world"), String::from("world"))

// Works with variables too (use & to get &str in script mode)
let slice: &str = &"test"
y := s!(slice)
eq!(y, String::from("test"))

put!("s! macro test passed")
