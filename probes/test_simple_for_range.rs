#!/usr/bin/env rust
sum := 0

for i in 0…5 {
	put!(i)
	sum+=i
}
eq!( sum,1+2+3+4+5);

for i in 0...5 {
	put!(i)
	sum-=i
}
eq!( sum, 0);
put!("All checks passed!\n")