#!/usr/bin/env rustc
// Test not operator with truthiness evaluation
assert_eq!( not 0 , true);
assert_eq!( not "" , true);
assert_eq!( not "x" , false);
assert_eq!( not []int{} , true);
assert_eq!( not []int{1,2} , false);
assert_eq!( not true , false);
assert_eq!( not false , true);

// Test with variables  
x := 0
assert_eq!( not x , true);

s := "hello"
assert_eq!( not s , false);

empty := ""
assert_eq!( not empty , true);

slice := []int{1,2,3}
assert_eq!( not slice , false);

empty_slice := []int{}
assert_eq!( not empty_slice , true);

// Test with floats (literals work)
assert_eq!( not 0.0 , true);
assert_eq!( not 3.14 , false);

// Test with different integer types (these should work with type info)
var zero_int8 int8 = 0
assert_eq!( not zero_int8 , true);

var nonzero_int64 int64 = 42
assert_eq!( not nonzero_int64 , false);

// TODO: Add support for maps, pointers, channels in 'not' operator
// These work fine with runtime truthiness in 'if' statements (see test_truthy.goo)
// but need transform context type information for 'not' operator support