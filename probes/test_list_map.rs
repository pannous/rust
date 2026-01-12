#!/usr/bin/env rustc
// import "slices"

# eq!( [1,2,3].apply(x=>x*2) , [2,4,6]  ); // (type []any) TODO: []int auto

xs := []int{1, 2, 3}
ys := xs.apply(x=>x*2) 
eq!( ys , []int{2, 4, 6});

