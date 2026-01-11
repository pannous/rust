#!/usr/bin/env rustc
import os

// Test 1-indexed array access
z := []rune{'a', 'b', 'c'}
if z#1 ≠ 'a' { print("First element via #1 fails"); exit() }
// assert_eq!( z#2 , 'b' ); // Second element
// assert_eq!( z#3 , 'c' );

// Test with numbers
nums := []int{10, 20, 30, 40}
// assert_eq!( nums#1 , 10);
// assert_eq!( nums#4 , 40);

// Test with expressions
idx := 2
if z#idx ≠ 'b' { print("First element via #idx fails"); exit() }
// if not (z#idx == 'b') { print("First element via #idx fails"); exit() }
put("All tests passed successfully!")