#!/usr/bin/env rust
// import "strconv"

fn testAssignment() error {
	try val := strconv.Atoi("42")
	put!("Converted value:", val)
	
	try val2 := strconv.Atoi("invalid")
	put!("This should not print:", val2)
	
	return nil
}

fn main() {
	err := testAssignment()
	if err != nil {
		put!("Error caught:", err.Error())
	} else {
		put!("No error - unexpected!")
	}
}