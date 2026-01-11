#!/usr/bin/env rustc

// Test hash with both if and check  
z := []rune{'a', 'b', 'c'}

// This should work
if z#1 != 'a' {
	panic("if failed")
}

// Does check work?
assert_eq!( z#1 , 'a');
