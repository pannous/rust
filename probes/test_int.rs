#!/usr/bin/env rust
// test int type alias for i64

var x: int = 42
put!(x)

var y: int = -100
put!(y)

// test arithmetic
var z: int = x + y
put!(z)

// test large values
var big: int = 9_223_372_036_854_775_807
put!(big)
