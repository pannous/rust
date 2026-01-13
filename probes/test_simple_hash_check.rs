#!/usr/bin/env rust

// Test hash with both if and assert!() 
z := []rune{'a', 'b', 'c'}

// This should work
if z#1 != 'a' {
	panic("if failed")
}

// Does assert!()work?
eq!( z#1 , 'a');
