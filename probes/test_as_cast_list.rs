#!/usr/bin/env rust

// any_list := @[1, "two", 3.0, true, nil]
// any_list := vec![1, "two", 3.0, true, nil]
let any_list : Vec<Val> = @[1, "two", 3.0, true, nil]

eq!( any_list[0] as int , 1 ); // neccessary?
eq!( any_list[1] as string , "two" ); // neccessary?
eq!( any_list[2] as float , 3.0 ); // neccessary?
eq!( any_list[3] as bool , true ); // neccessary?
