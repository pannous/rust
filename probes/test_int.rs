#!/usr/bin/env rust
// test int and float type aliases

var x: int = 42
put!(x)

var y: int = -100
put!(y)

var z: int = x + y
put!(z)

var big: int = 9_223_372_036_854_775_807
put!(big)

// test float alias for f64
var pi: float = 3.14159265358979
put!(pi)

var e: float = 2.71828
put!(e)

var sum: float = pi + e
put!(sum)
