#!/usr/bin/env rust
eq!( 1…3 , [1, 2, 3] ); // assert!()range syntax
eq!( 'a'…'c' , ['a', 'b', 'c'] ); // assert!()character range syntax
put!("Range checks passed!\n")