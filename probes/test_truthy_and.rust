#!/usr/bin/env rust

// Test numeric truthiness
if 0 {
	put!("FAIL: 0 should be falsy")
	assert!(false)
} else {
	put!("PASS: 0 is falsy")
}

if 42 {
	put!("PASS: 42 is truthy")
} else {
	put!("FAIL: 42 should be truthy")
	assert!(false)
}

// Test string truthiness
if "" {
	put!("FAIL: empty string should be falsy")
	assert!(false)
} else {
	put!("PASS: empty string is falsy")
}

if "hello" {
	put!("PASS: non-empty string is truthy")
} else {
	put!("FAIL: non-empty string should be truthy")
	assert!(false)
}

// Test slice truthiness (empty vec is falsy in script mode)
let nilSlice: Vec<i32> = vec![]
if nilSlice {
	put!("FAIL: empty slice should be falsy")
	assert!(false)
} else {
	put!("PASS: empty slice is falsy")
}

let emptySlice: Vec<i32> = vec![]
if emptySlice {
	put!("FAIL: empty slice should be falsy")
	assert!(false)
} else {
	put!("PASS: empty slice is falsy")
}

nonEmptySlice := @[1, 2, 3]
if nonEmptySlice {
	put!("PASS: non-empty slice is truthy")
} else {
	put!("FAIL: non-empty slice should be truthy")
	assert!(false)
}

// Test boolean (should work normally)
if true {
	put!("PASS: true is truthy")
} else {
	put!("FAIL: true should be truthy")
	assert!(false)
}

if false {
	put!("FAIL: false should be falsy")
	assert!(false)
} else {
	put!("PASS: false is falsy")
}

// Test Option truthiness
let nilOpt: Option<i32> = None
if nilOpt {
	put!("FAIL: None should be falsy")
	assert!(false)
} else {
	put!("PASS: None is falsy")
}

someOpt := Some(42)
if someOpt {
	put!("PASS: Some is truthy")
} else {
	put!("FAIL: Some should be truthy")
	assert!(false)
}

put!("All truthy_and tests passed!")
