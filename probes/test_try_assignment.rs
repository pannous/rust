#!/usr/bin/env rust
// import "strconv"

fn testAssignment() error {
	try val := strconv.Atoi("42")
	println("Converted value:", val)
	
	try val2 := strconv.Atoi("invalid")
	println("This should not print:", val2)
	
	return nil
}

fn main() {
	err := testAssignment()
	if err != nil {
		println("Error caught:", err.Error())
	} else {
		println("No error - unexpected!")
	}
}