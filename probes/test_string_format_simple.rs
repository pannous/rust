#!/usr/bin/env rust
result := "my %v modulo" % "cool"
put!("Result: {}", result)
eq!( result , "my cool modulo");