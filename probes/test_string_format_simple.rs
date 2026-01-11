#!/usr/bin/env rustc
result := "my %v modulo" % "cool"
printf("Result: %s\n", result)
assert_eq!( result , "my cool modulo");