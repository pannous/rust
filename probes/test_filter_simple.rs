#!/usr/bin/env rustc
import "slices"

xs := [1, 2, 3, 4, 5]
evens := xs.filter(x => x%2 == 0)
eq!( evens , [2, 4]);

odds := xs.filter(x => x%2 == 1) 
eq!( odds , [1, 3, 5]);

print("Filter tests passed")