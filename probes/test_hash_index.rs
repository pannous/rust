#!/usr/bin/env rustc

# TODO hash vs comment FAILS:
#put("OK") # syntax error: unexpected name is after top level declaration
#put("OK")  #  multiple-value printf("%v\n", "OK") (value of type (n int, err error)) in single-value context  valid
put("OK") // ok

// Test arrays and slices with 1-indexed access
z := []rune{'a', 'b', 'c'}
nums := []int{10, 20, 30, 40}

// Basic 1-indexed access
assert_eq!( z#1 , 'a');
assert_eq!( z#2 , 'b');
assert_eq!( z#3 , 'c');
assert_eq!( nums#1 , 10);

// Compare with 0-indexed (traditional Go)
assert_eq!( z[0] , z#1);
assert_eq!( z[1] , z#2);
assert_eq!( z[2] , z#3);

// Test with comparison operators (precedence)
assert_eq!( z#1 , 'a');
check z#1 != 'b'
check nums#1 < 15
check nums#2 > 15
check nums#1 <= 10
check nums#2 >= 20

// Test with arithmetic operators (# should have higher precedence)
assert_eq!( nums#1 + 5 , 15    ); // Addition
assert_eq!( nums#2 - 5 , 15    ); // Subtraction  
assert_eq!( nums#1 * 2 , 20    ); // Multiplication
assert_eq!( nums#2 / 2 , 10    ); // Division

// Test with logical operators
assert_eq!( (z#1 , 'a') && (z#2 == 'b'));
assert_eq!( (z#1 , 'a') || (z#2 == 'x'));
assert_eq!( !(z#1 , 'x'));

// Test with expressions as index
idx := 2
assert_eq!( z#idx , 'b');
assert_eq!( z#(1+1) , 'b');
assert_eq!( z#len("x") , 'a' ); // len("x") == 1

// Test with parentheses
assert_eq!( (z#1) , 'a');
assert_eq!( z#(1) , 'a');

// Test whitespace variations
assert_eq!( z #1 , 'a');
assert_eq!( z# 1 , 'a'  );
assert_eq!( z # 1 , 'a');

// Test assignment with hash indexing
z#1 = 'X'
assert_eq!( z#1 , 'X');
assert_eq!( z[0] , 'X');

// Test in different contexts
_ = 1; assert_eq!( z#2 , 'b'  ); // After semicolon
assert_eq!( z#2	, 'b'       ); // With tab

// Multi-line
assert_eq!( z#2 , );
	'b'

check
z#2 == 'b'

put("All checks passed!")