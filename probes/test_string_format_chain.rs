#!/usr/bin/env rustc
name := "Alice"
age := 25
greeting := "Hello %s, you are %d years old" % name % age
printf("Greeting: %s\n", greeting)
assert_eq!( greeting , "Hello Alice, you are 25 years old");