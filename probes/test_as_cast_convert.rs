#!/usr/bin/env rust
eq!( 1 as string , "1");
eq!( 1 as rune , '1');
eq!( '1' as int , 1);
eq!( 3 as float , 3.0);

put!("some tests OK;)")

eq!( 3.14 as int , 3);
eq!( 3.14 as string , "3.14");
eq!( "1" as int , 1 );
