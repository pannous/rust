#!/usr/bin/env rust
// Test not operator with truthiness evaluation
eq!( not 0 , true);
eq!( not "" , true);
eq!( not "x" , false);
eq!( not []int{} , true);
eq!( not []int{1,2} , false);
eq!( not true , false);
eq!( not false , true);

// Test with variables  
x := 0
eq!( not x , true);

s := "hello"
eq!( not s , false);

empty := ""
eq!( not empty , true);

slice := []int{1,2,3}
eq!( not slice , false);

empty_slice := []int{}
eq!( not empty_slice , true);

// Test with floats (literals work)
eq!( not 0.0 , true);
eq!( not 3.14 , false);

// Test with different integer types (these should work with type info)
let zero_int8 int8 = 0
eq!( not zero_int8 , true);

let nonzero_int64 int64 = 42
eq!( not nonzero_int64 , false);

// TODO: Add support for maps, pointers, channels in 'not' operator
// These work fine with runtime truthiness in 'if' statements (see test_truthy.goo)
// but need transform context type information for 'not' operator support