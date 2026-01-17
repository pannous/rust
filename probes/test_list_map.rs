#!/usr/bin/env rust
// import "slices"

# eq!( [1,2,3].apply(x=>x*2) , [2,4,6]  ); // (type []any) TODO: []int auto

xs := @[1, 2, 3]
ys := xs.apply(x=>x*2)
eq!( ys , @[2, 4, 6]);

