#!/usr/bin/env rustc
#import "strings" // auto import on demand WORKS!
#import "fmt" // auto import on demand WORKS!
#import "unicode" // auto import on demand WORKS!
#import "strconv" // for string conversion
#import "string_tools" // for string manipulation utilities  'cannot find package' <WHICH?>c TODO

// Check string methods and conversions THAT WORK!

assert_eq!( "a"+"1" , "a1");
assert_eq!( "a"+1 , "a1" );
assert_eq!( "hi".first() , "h");
assert_eq!( "hi".last() , "i");
assert_eq!( "hi".size() , 2);
assert_eq!( "hi".length() , 2);
assert_eq!( "hi".reverse() , "ih");
assert_eq!( "hello".reverse() , "olleh");
check "hello"

assert_eq!( "hi".contains("h") , true);
assert_eq!( "hi".contains("x") , false);
assert_eq!( "hi".indexOf("h") , 0);
assert_eq!( "hi".indexOf("x") , -1);
assert_eq!( "hi".indexOf("i") , 1);
assert_eq!( "hi"[1:] , "i" ); // index -1 (constant of type int) must not be negative
assert_eq!( "hi".from(1) , "i");
assert_eq!( "hi".to(1) , "h");
assert_eq!( "hello".sub(1,3) , "el" ); // sub(start, end) is inclusive of start and exclusive of end
assert_eq!( "hello".replace("l", "x") , "hexxo");
assert_eq!( "hello".toUpper() , "HELLO");
assert_eq!( "hello".toLower() , "hello");
assert_eq!( "hello".upper() , "HELLO" ); // toUpper is an alias for upper, show 💡
assert_eq!( "hello".lower() , "hello");
assert_eq!( "hello".upperCase() , "HELLO");
assert_eq!( "hello".lowerCase() , "hello");
assert_eq!( "hello".capitalize() , "Hello");
assert_eq!( "hello".title() , "Hello");
assert_eq!( "hello".trim() , "hello");
assert_eq!( " hello ".trim() , "hello");
assert_eq!( " hello ".trim() , "hello");
assert_eq!( "hello".join("-") , "h-e-l-l-o");
assert_eq!( "hello".join("") , "hello");
assert_eq!( "hello".startsWith("he") , true);
assert_eq!( "hello".startsWith("lo") , false);
assert_eq!( "hello".endsWith("lo") , true);
assert_eq!( "hello".endsWith("he") , false);

put("hello".split("")) // [h e l l o]  without quotes ?!
assert_eq!( "hello".split("l") , ["he", "", "o"] // as string[] ); // split returns a list of strings
assert_eq!( "hello".split("") , ["h", "e", "l", "l", "o"]);
assert_eq!( "hello".splits() , ["h", "e", "l", "l", "o"]);


printf("All checks passed!\n")