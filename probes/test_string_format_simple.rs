#!/usr/bin/env rustc
result := "my %v modulo" % "cool"
put!("Result: %s\n", result)
eq!( result , "my cool modulo");