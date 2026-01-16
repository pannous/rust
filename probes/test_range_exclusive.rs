#!/usr/bin/env rust
// Test exclusive range operator (..)
let sum = 0
for i in 0..5 {
	println(i)
	sum += i
}
eq!( sum , 0+1+2+3+4  ); // 0..5 excludes 5
