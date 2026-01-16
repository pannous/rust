#!/usr/bin/env rust
name := "Alice"
age := 25
greeting := "Hello %s, you are %d years old" % name % age
put!("Greeting: {}", greeting)
eq!( greeting , "Hello Alice, you are 25 years old");