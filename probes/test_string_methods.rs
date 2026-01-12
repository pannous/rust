#!/usr/bin/env rustc
// #import "strings" // auto import on demand WORKS!
// #import "fmt" // auto import on demand WORKS!
// #import "unicode" // auto import on demand WORKS!
// #import "strconv" // for string conversion
// #import "string_tools" // for string manipulation utilities  'cannot find package' <WHICH?>c TODO

// assert!()string methods and conversions THAT WORK!

eq!( "a"+"1" , "a1");
eq!( "a"+1 , "a1" );
eq!( "hi".first() , "h");
eq!( "hi".last() , "i");
eq!( "hi".size() , 2);
eq!( "hi".length() , 2);
eq!( "hi".reverse() , "ih");
eq!( "hello".reverse() , "olleh");
assert!()"hello"

eq!( "hi".contains("h") , true);
eq!( "hi".contains("x") , false);
eq!( "hi".indexOf("h") , 0);
eq!( "hi".indexOf("x") , -1);
eq!( "hi".indexOf("i") , 1);
eq!( "hi"[1:] , "i" ); // index -1 (constant of type int) must not be negative
eq!( "hi".from(1) , "i");
eq!( "hi".to(1) , "h");
eq!( "hello".sub(1,3) , "el" ); // sub(start, end) is inclusive of start and exclusive of end
eq!( "hello".replace("l", "x") , "hexxo");
eq!( "hello".toUpper() , "HELLO");
eq!( "hello".toLower() , "hello");
eq!( "hello".upper() , "HELLO" ); // toUpper is an alias for upper, show 💡
eq!( "hello".lower() , "hello");
eq!( "hello".upperCase() , "HELLO");
eq!( "hello".lowerCase() , "hello");
eq!( "hello".capitalize() , "Hello");
eq!( "hello".title() , "Hello");
eq!( "hello".trim() , "hello");
eq!( " hello ".trim() , "hello");
eq!( " hello ".trim() , "hello");
eq!( "hello".join("-") , "h-e-l-l-o");
eq!( "hello".join("") , "hello");
eq!( "hello".startsWith("he") , true);
eq!( "hello".startsWith("lo") , false);
eq!( "hello".endsWith("lo") , true);
eq!( "hello".endsWith("he") , false);

put!("hello".split("")) // [h e l l o]  without quotes ?!
eq!( "hello".split("l") , ["he", "", "o"] // as string[] ); // split returns a list of strings
eq!( "hello".split("") , ["h", "e", "l", "l", "o"]);
eq!( "hello".splits() , ["h", "e", "l", "l", "o"]);


put!("All checks passed!\n")