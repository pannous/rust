#!/usr/bin/env rust
// Comprehensive as cast test
val1 := 1 as string
put!("1 as string: %q\n", val1) 

val2 := 1 as rune
put!("1 as rune: %c (%d)\n", val2, val2)

val3 := '1' as int
put!("'1' as int: %d\n", val3)

val4 := 3 as float
put!("3 as float: %f\n", val4)

put!("All as cast tests completed\n")