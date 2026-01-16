#!/usr/bin/env rust
// import "fmt"
// import "errors"

fn niceFunction() error{
	return nil
}


fn failingFunction() error {
	return errors.New("try to catch me;)")
}

fn testFunction() error {
	try niceFunction()
	try failingFunction()
	put!("This should not be reached")
	return nil  
}

fn main() {
	err := testFunction()
	if err != nil {
		println("Success! Error propagated:", err.Error())
	} else {
		println("No error returned")
	}
}