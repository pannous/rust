#!/usr/bin/env rust

// import "fmt"

let ptr *int
if ptr == ø {
	put!("ptr is nil using ø")
}

let slice []int
if slice == ø {
	put!("slice is nil using ø")
}

let m map[string]int
if m == ø {
	put!("map is nil using ø")
}

// Test assignment
ptr = ø
if ptr == ø {
	put!("ptr assigned to ø works")
}

// Test return
result := getPtr()
if result == ø {
	put!("function returned ø")
}


fn getPtr() *int {
	return ø
}