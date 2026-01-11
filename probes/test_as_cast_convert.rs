#!/usr/bin/env rustc
import "strconv"
#
// see as_cast_transform.go

assert_eq!( 1 as string , "1");
assert_eq!( 1 as rune , '1');
assert_eq!( '1' as int , 1);
assert_eq!( 3 as float , 3);

printf("some tests OK;)")

assert_eq!( 3.14 as int , 3);
assert_eq!( 3.14 as string , "3.14");
// TODO - Now working!
# assert_eq!( "1" as int , 1 # HARD! later?);
