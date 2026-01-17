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

if -1 {
    put!("PASS: -1 is truthy")
} else {
    put!("FAIL: -1 should be truthy")
    assert!(false)
}

// Test float truthiness
if 0.0 {
    put!("FAIL: 0.0 should be falsy")
    assert!(false)
} else {
    put!("PASS: 0.0 is falsy")
}

if 3.14 {
    put!("PASS: 3.14 is truthy")
} else {
    put!("FAIL: 3.14 should be truthy")
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

// Test String type truthiness
if String::new() {
    put!("FAIL: empty String should be falsy")
    assert!(false)
} else {
    put!("PASS: empty String is falsy")
}

if String::from("hello") {
    put!("PASS: non-empty String is truthy")
} else {
    put!("FAIL: non-empty String should be truthy")
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

// Test unsigned integers
if 0u8 {
    put!("FAIL: 0u8 should be falsy")
    assert!(false)
} else {
    put!("PASS: 0u8 is falsy")
}

if 1u64 {
    put!("PASS: 1u64 is truthy")
} else {
    put!("FAIL: 1u64 should be truthy")
    assert!(false)
}

// Test Vec truthiness
if Vec::<i32>::new() {
    put!("FAIL: empty Vec should be falsy")
    assert!(false)
} else {
    put!("PASS: empty Vec is falsy")
}

if vec![1, 2, 3] {
    put!("PASS: non-empty Vec is truthy")
} else {
    put!("FAIL: non-empty Vec should be truthy")
    assert!(false)
}

// Test Option truthiness
if None::<i32> {
    put!("FAIL: None should be falsy")
    assert!(false)
} else {
    put!("PASS: None is falsy")
}

if Some(42) {
    put!("PASS: Some is truthy")
} else {
    put!("FAIL: Some should be truthy")
    assert!(false)
}

// Test else-if chain
x := 5
if x {
    put!("PASS: x equals 5 ≠ 0")
} else if x == 5 {
	put!("PASS: x equals 5")
} else {
    put!("FAIL: else-if not working correctly")
    assert!(false)
}

let y : i32? =1
if y {
    put!("PASS: y is truthy")
} else {
    put!("FAIL: y should be truthy")
    assert!(false)
}

let z : i32? = None
if z {
    put!("FAIL: z should be falsy")
    assert!(false)
} else {
    put!("PASS: z is falsy")
}

put!("All truthy tests passed!")
