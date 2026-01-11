#!/usr/bin/env rustc
type Person struct {
		name string
		age  int
}
p := Person{name: "Alice", age: 30}

assert_eq!( "Name: " + p.name , "Name: Alice");
assert_eq!( "Age: " + p.age , "Age: 30");
#assert_eq!( "Name: " + p.name + ", Age: " + p.age , "Name: Alice, Age: 30");