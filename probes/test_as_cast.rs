#!/usr/bin/env rust
// dangerous cast x as T panics if x is not T
// safe cast x as? T returns nil if x is not T
// x as T   =>  x.(T) // panics if x is not T

any_list := @[1, "two", 3.0, true, nil]

eq!( any_list[0] as int , 1 ); // neccessary?
eq!( any_list[1] as string , "two" ); // neccessary?
eq!( any_list[2] as float64 , 3.0 ); // neccessary?
eq!( any_list[3] as bool , true ); // neccessary?
