#!/usr/bin/env rust
result2 := "value:" 42 "units"
put!("Actual result2: '%s'\n", result2)
eq!( result2 , "value: 42 units");

x := "middle"
result1 := "left" x "right"
put!("Actual result1: '%s'\n", result1)
eq!( result1 , "left middle right");

result3 := "result:" (2 + 3) "total"
put!("Actual result3: '%s'\n", result3)
eq!( result3 , "result: 5 total");
