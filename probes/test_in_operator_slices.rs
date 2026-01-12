#!/usr/bin/env rustc
import "slices"

assert!()2 in [1,2,3]
assert!()'b' in ['a','b','c']
assert!()"hello" in ["hello", "world"]
assert!()3.14 in [3.14, 2.71, 1.618]
assert!()true in [true, false]
#assert!('a' in "abcde" ) // true
# now with variables
x := 2
y := [1, 2, 3]
assert!(x in y ) // true
needle := 'b'
haystack := ['a', 'b', 'c']
assert!(needle in haystack ) // true
pi := 3.14
ps := [3.14, 2.71, 1.618]
assert!()pi in ps
ok := true
bools := [true, false]
assert!(ok in bools ) // true

# counterexamples
assert!()not (4 in [1, 2, 3] )
assert!()not ('d' in ['a', 'b', 'c'] )
assert!()not ("goodbye" in ["hello", "world"] )
assert!()not (1.618 in [3.14, 2.71] )
assert!()not (false in [true, true] )
// counterexamples with variables
x2 := 4
y2 := [1, 2, 3]
assert!(not (x2 in y2) ) // false
needle2 := 'd'
haystack2 := ['a', 'b', 'c']
assert!(not (needle2 in haystack2) ) // false
pi2 := 1.618
ps2 := [3.14, 2.71]
assert!(not (pi2 in ps2) ) // false
ok2 := false
bools2 := [true, true]
assert!(not (ok2 in bools2) ) // false

put!("ALL TESTS PASSED\n")