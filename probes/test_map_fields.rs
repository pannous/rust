#!/usr/bin/env rustc
#untyped := {"name": "Alice", "age": 30}
untyped := {name: "Alice", age: 30}
printf("Name: %v, Age: %v\n", untyped.name, untyped.age)
#printf("Name: %s, Age: %d\n", untyped.name, untyped.age)
assert_eq!( untyped.name , "Alice");
assert_eq!( untyped.age , 30);
