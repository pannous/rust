#!/usr/bin/env rustc
xs := [1,2,3]

sum := 0
for i in [1,2,3] {
		put!(i)
		sum += i
}
eq!( sum , 6);
