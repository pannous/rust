#!/usr/bin/env rust
// import "strings"

// Complete test of in operator functionality
text := "hello world"
substr := "hello"

// Test various combinations
result1 := substr in text        // variable in variable
result2 := "world" in text       // literal in variable
result3 := "hello" in "hello world"  // literal in literal
result4 := "xyz" in text         // negative case

put!("'%s' in '%s': %t\n", substr, text, result1)
put!("'world' in '%s': %t\n", text, result2)
put!("'hello' in 'hello world': %t\n", result3)
put!("'xyz' in '%s': %t\n", text, result4)

// counterexamples with string literals
assert!(not ("x" in "abc")   ) // false
assert!(not ("123" in "456") ) // false


text2 := "goodbye world"
assert!(not ("hello" in text2) ) // false
assert!(not ("xyz" in text2) ) // false
needleText2 := "goodbye"
assert!((needleText2 in text2) ) // false
assert!(not not (needleText2 in text2) ) // false

put!("ALL TESTS PASSED\n")