#!/usr/bin/env rustc
#untyped := {"name": "Alice", "age": 30}
untyped := {name: "Alice", age: 30}
put!("Name: %v, Age: %v\n", untyped.name, untyped.age)
#put!("Name: %s, Age: %d\n", untyped.name, untyped.age)
eq!( untyped.name , "Alice");
eq!( untyped.age , 30);
