#!/usr/bin/env rustc
import "fmt"

// Test string to character comparison
s := "你"
c := '你'

put!("String: %s\n", s)
put!("Char: %c\n", c)
put!("s == c: %t\n", s == c)