#!/usr/bin/env rustc
put!("Type of 42: %v\n", typeof(42)) // todo why 'untyped int
fmt.Println(typeof(42))
eq!( typeof(42) , "untyped int");
