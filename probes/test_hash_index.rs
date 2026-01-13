#!/usr/bin/env rust

# TODO hash vs comment FAILS:
#put!("OK") # syntax error: unexpected name is after top level declaration
#put!("OK")  #  multiple-value put!("%v\n", "OK") (value of type (n int, err error)) in single-value context  valid
put!("OK") // ok

// Test arrays and slices with 1-indexed access
z := []rune{'a', 'b', 'c'}
nums := []int{10, 20, 30, 40}

// Basic 1-indexed access
eq!( z#1 , 'a');
eq!( z#2 , 'b');
eq!( z#3 , 'c');
eq!( nums#1 , 10);

// Compare with 0-indexed (traditional Go)
eq!( z[0] , z#1);
eq!( z[1] , z#2);
eq!( z[2] , z#3);

// Test with comparison operators (precedence)
eq!( z#1 , 'a');
assert!()z#1 != 'b'
assert!()nums#1 < 15
assert!()nums#2 > 15
assert!()nums#1 <= 10
assert!()nums#2 >= 20

// Test with arithmetic operators (# should have higher precedence)
eq!( nums#1 + 5 , 15    ); // Addition
eq!( nums#2 - 5 , 15    ); // Subtraction  
eq!( nums#1 * 2 , 20    ); // Multiplication
eq!( nums#2 / 2 , 10    ); // Division

// Test with logical operators
eq!( (z#1 , 'a') && (z#2 == 'b'));
eq!( (z#1 , 'a') || (z#2 == 'x'));
eq!( !(z#1 , 'x'));

// Test with expressions as index
idx := 2
eq!( z#idx , 'b');
eq!( z#(1+1) , 'b');
eq!( z#len("x") , 'a' ); // len("x") == 1

// Test with parentheses
eq!( (z#1) , 'a');
eq!( z#(1) , 'a');

// Test whitespace variations
eq!( z #1 , 'a');
eq!( z# 1 , 'a'  );
eq!( z # 1 , 'a');

// Test assignment with hash indexing
z#1 = 'X'
eq!( z#1 , 'X');
eq!( z[0] , 'X');

// Test in different contexts
_ = 1; eq!( z#2 , 'b'  ); // After semicolon
eq!( z#2	, 'b'       ); // With tab

// Multi-line
eq!( z#2 , );
	'b'

check
z#2 == 'b'

put!("All checks passed!")