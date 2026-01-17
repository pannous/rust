#!/usr/bin/env rust
// Test ellipsis range syntax - collect to vec for comparison
eq!( (1…4).collect::<Vec<_>>() , vec![1, 2, 3] ); // range 1..4 gives [1,2,3]
eq!( ('a'…'d').collect::<Vec<_>>() , vec!['a', 'b', 'c'] ); // char range
put!("Range checks passed!\n")