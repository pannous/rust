#!/usr/bin/env rust

for i := 0; i < 5; i++ {
	if i == 2 {
		continue
	}
	println(i)
}
