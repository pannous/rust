#!/usr/bin/env rust

let x = 42
let y = 41 + 1
eq!( x , y);
list1 := @[1,2]
list2 := @[1,2]
if list1 == list2 {
	put!("Test passed: lists are equal")
} else {
	put!("Test failed: lists are not equal")
}
a := @[1, 2, 3]
b := @[1, 2, 3]
eq!( a , b);

eq!( list1 , list2);

put!("Test passed: a and b are equal")



// Test what algorithm different types use
// We can infer this from what comparisons work

// Basic types that should be comparable
put!("Testing basic comparisons:")
put!("int: {} == {}: {}\n", 1, 1, 1 == 1)
put!("string: {} == {}: {}\n", "a", "a", "a" == "a")
put!("bool: {} == {}: {}\n", true, true, true == true)

// Function types (should not be comparable except to nil)
# let f1 fn()
# let f2 fn()
# put!("nil fn == nil fn: %v\n", f1 == f2)

// What about slices of functions?
put!("Testing slice element types:")

// Try to compare int slices (this might tell us if they're going through runtime)
a1 := @[1]  // no new variables on left side of :=
b1 := @[1]
put!("@[1] == @[1]: {}\n", a1 == b1)

// Test what happens with int comparison
# a := 1 cannot use 1 (untyped int constant) as []any value in assignment

// Test arrays for comparison
arr1 := @[1, 2]
arr2 := @[1, 2]
arr3 := @[3, 4]

put!("Array comparison: {:?} == {:?}: {}\n", arr1, arr2, arr1 == arr2)
put!("Array comparison: {:?} == {:?}: {}\n", arr1, arr3, arr1 == arr3)

// Test empty slice lengths
let empty1: Vec<i32> = vec![]
let empty2: Vec<i32> = vec![]

put!("len(empty1): {}\n", empty1.len())
put!("len(empty2): {}\n", empty2.len())
put!("len(empty1) == len(empty2): {}\n", empty1.len() == empty2.len())
put!("empty1 == empty2: {}\n", empty1 == empty2)

// Test more empty slices (Rust has no nil slices)
let nil1: Vec<i32> = vec![]
let nil2: Vec<i32> = vec![]
put!("nil1 == nil2: {}\n", nil1 == nil2)
put!("len(nil1): {}, len(nil2): {}\n", nil1.len(), nil2.len())

// Test empty slices comparison
put!("nil1 == empty1: {}\n", nil1 == empty1)
put!("All tests completed successfully.\n")