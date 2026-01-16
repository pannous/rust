#!/usr/bin/env rust
// Test only basic methods that don't need imports
eq!( "a"+"1" , "a1");
eq!( "a"+1 , "a1" ); // invalid operation: "a" + 1 (mismatched types untyped string and untyped int)
eq!( "hi".first() , "h");
eq!( "hi".last() , "i");
eq!( "hi".size() , 2);
eq!( "hi".length() , 2);

// #struct Person {
// #		name string
// #		age  int
// #}
// #p := Person{name: "Alice", age: 30}
// #eq!( "Name: " + p.name + ", Age: " + p.age , "Name: Alice, Age: 30");