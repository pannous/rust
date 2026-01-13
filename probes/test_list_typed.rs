#!/usr/bin/env rust
xs := [1, 2, 3]
ys := []int{1, 2, 3}
eq!( xs , ys ); // should return true. currently: (mismatched types []any and []int)
