#!/usr/bin/env rustc
assert_eq!( ("a" , 'a')  ); // ✅ Now works! Transformed to "a" == string('a')
assert_eq!( ("a" , 'a') ); // ✅ String/rune comparison now supported!

// Test only basic methods that don't need imports
assert_eq!( "a"+"1" , "a1");
assert_eq!( "a"+1 , "a1" ); // invalid operation: "a" + 1 (mismatched types untyped string and untyped int)
assert_eq!( "hi".first() , "h");
assert_eq!( "hi".last() , "i");
