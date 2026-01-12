#!/usr/bin/env rustc
// import os

// Test 1-indexed array access
z := []rune{'a', 'b', 'c'}
if z#1 ≠ 'a' { print("First element via #1 fails"); exit() }
// eq!( z#2 , 'b' ); // Second element
// eq!( z#3 , 'c' );

// Test with numbers
nums := []int{10, 20, 30, 40}
// eq!( nums#1 , 10);
// eq!( nums#4 , 40);

// Test with expressions
idx := 2
if z#idx ≠ 'b' { print("First element via #idx fails"); exit() }
// if not (z#idx == 'b') { print("First element via #idx fails"); exit() }
put!("All tests passed successfully!")