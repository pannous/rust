#!/usr/bin/env rustc
#type Person struct {
class Person {
	name string
	age  int
}

untyped := {name: "Alice", age: 30}
person := Person{name: "Alice", age: 30}
printf("Name: %s, Age: %d\n", person.name, person.age)
#printf("Name: %s, Age: %d\n", untyped.name, untyped.age)
assert_eq!( person.name , untyped.name);
assert_eq!( person.age , untyped.age);