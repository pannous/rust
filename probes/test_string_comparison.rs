#!/usr/bin/env rustc
eq!( ("a" , 'a')  ); // ✅ Now works! Transformed to "a" == string('a')
eq!( ("a" , 'a') ); // ✅ String/rune comparison now supported!

// Test only basic methods that don't need imports
eq!( "a"+"1" , "a1");
eq!( "a"+1 , "a1" ); // invalid operation: "a" + 1 (mismatched types untyped string and untyped int)
eq!( "hi".first() , "h");
eq!( "hi".last() , "i");
