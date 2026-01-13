#!/usr/bin/env rust
type Person struct {
		name string
		age  int
}
p := Person{name: "Alice", age: 30}

eq!( "Name: " + p.name , "Name: Alice");
eq!( "Age: " + p.age , "Age: 30");
#eq!( "Name: " + p.name + ", Age: " + p.age , "Name: Alice, Age: 30");