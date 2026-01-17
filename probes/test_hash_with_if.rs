#!/usr/bin/env rust

// Test 1-indexed array access with if statements
z := @['a', 'b', 'c']
if z#1 != 'a' { panic("First element failed") }
if z#2 != 'b' { panic("Second element failed") }
if z#3 != 'c' { panic("Third element failed") }
