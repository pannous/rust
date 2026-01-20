#!/usr/bin/env rust

// Simple truthy test - integers and strings only

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

put!("All tests passed!")
