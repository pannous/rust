#!/usr/bin/env rust
//   string concatenation requires an owned `String` on the left

// Test string + number concatenation feature

result0 := "a" + "0"
eq!( result0 , "a0");
put!("Test 0: " + result0)

// Basic string + integer
result1 := "a" + 1
eq!( result1 , "a1");
put!("Test 1: " + result1)

// String + larger integer
result2 := "value: " + 42
eq!( result2 , "value: 42");
put!("Test 2: " + result2)

// Integer + string (requires explicit conversion)
result3 := 1.to_string() + "b"
eq!( result3 , "1b");
put!("Test 3: " + result3)

// String + negative integer
result4 := "count: " + (-5)
eq!( result4 , "count: -5");
put!("Test 4: " + result4)

// Integer + string with spaces (requires explicit conversion)
result5 := 123.to_string() + " items"
eq!( result5 , "123 items");
put!("Test 5: " + result5)

// String + float (should work with numeric types)
result6 := "pi is " + 3.14159
eq!( result6 , "pi is 3.14159");
put!("Test 6: " + result6)

// Chained concatenation
result7 := "prefix" + 1 + 2 + "suffix"
eq!( result7 , "prefix12suffix");
put!("Test 7: " + result7)

result8 := "a " + true
eq!( result8 , "a ✔️");
put!("Test 8: " + result8)

result9 := "a " + false
eq!( result9 , "a ✖️" );
put!("Test 9: " + result9)


put!("All string concatenation tests passed!")