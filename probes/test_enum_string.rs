#!/usr/bin/env rust
enum State { OK, ERROR, PENDING }

println("Testing enum String() method:")

// Test normal cases
var s State = OK
put!("OK.String() = %v\n", s)
eq!( s as string , "OK");

s = ERROR
put!("ERROR.String() = %s\n", s.String())
eq!( s as string , "ERROR");

s = PENDING
put!("PENDING.String() = %s\n", s.String())
eq!( s as string , "PENDING");

// Test unknown value (should return "UNKNOWN")
try{
	s = State(999)
	put!("State(999).String() = %s\n", s.String())
} catch err{
	put!("OK! Caught error for unknown enum value:", err)
}
// Test direct string formatting
put!("Direct formatting: %s\n", OK)

put!("All tests passed!\n")