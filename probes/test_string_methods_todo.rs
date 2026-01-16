#!/usr/bin/env rust
// import "strconv" // for string conversion
//#import "strings" // auto import on demand WORKS!
//#import "fmt" // auto import on demand WORKS!
//#import "unicode" // auto import on demand WORKS!
//#import "string_tools" // for string manipulation utilities  'cannot find package' <WHICH?>c TODO


// NON WORKING

fn main() {
	chars := ['h', 'e', 'l', 'l', 'ø'] 
	runes := []rune("hello")
	put!(typeof(runes)) // prints "[]rune"
	put!("%q\n", runes)
	put!(runes) // [104 101 108 108 111]
	//put!("hello".runes())
	eq!( "hellø".runes() , chars);
	eq!( "hellø".runes() , ['h', 'e', 'l', 'l', 'ø']);

//	i := 2
//eq!( "你" , '你');
//eq!( "你好"#2 , '好' ); // charAt returns a rune at the given index
//eq!( "你好"#i , '好');
bytos := []byte{104, 101, 108, 108, 111}
eq!( "hello".bytes() , bytos ); // returns a list of bytes (ASCII values)

//byts := []int("你好") // nah, ok
byts := []byte("你好") // [228 189 160 229 165 189] OK
put!(byts) // prints [104 101 108 108 111]
eq!( "你好".bytes() , []byte{228, 189, 160, 229, 165, 189} ); // returns a list of bytes (UTF-8 encoded values)

//invalid use of [...] array (outside a composite literal)
eq!( "hello".codePoints() , []int{104, 101, 108, 108, 111} ); // returns a list of code points (Unicode values)
eq!( "hello".runes() , []rune{'h', 'e', 'l', 'l', 'o'});
eq!( "42".toInt() , 42);
eq!( "42".toInt(10) , 42);
eq!( "42".toInt(16) , 66 ); // 42 in hex is
eq!( "42".toInt(2) , 0 ); // 42 in binary is 101010, which is not a valid binary number
eq!( "101010".toInt(2) , 42 ); // 101010 in binary is 42
eq!( "42".toFloat() , 42.0);
eq!( "42.5".toFloat() , 42.5);
#eq!( "a" , 'a');

put!("All checks passed!\n")
}