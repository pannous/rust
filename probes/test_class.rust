#!/usr/bin/env rust
#type Person struct {
class Person {
	name string
	age  int
}

untyped := {name: "Alice", age: 30}
person := Person{name: "Alice", age: 30}
put!("Name: %s, Age: %d\n", person.name, person.age)
#put!("Name: %s, Age: %d\n", untyped.name, untyped.age)
eq!( person.name , untyped.name);
eq!( person.age , untyped.age);