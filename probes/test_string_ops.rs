#!/usr/bin/env rustc
// Test only basic methods that don't need imports
assert_eq!( "a"+"1" , "a1");
assert_eq!( "a"+1 , "a1" ); // invalid operation: "a" + 1 (mismatched types untyped string and untyped int)
assert_eq!( "hi".first() , "h");
assert_eq!( "hi".last() , "i");
assert_eq!( "hi".size() , 2);
assert_eq!( "hi".length() , 2);

#struct Person {
#		name string
#		age  int
#}
#p := Person{name: "Alice", age: 30}
#assert_eq!( "Name: " + p.name + ", Age: " + p.age , "Name: Alice, Age: 30");