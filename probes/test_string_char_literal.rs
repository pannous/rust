#!/usr/bin/env rust
// import "fmt"

// Test direct literal comparison
result1 := "你" == '你'
result2 := '你' == "你"

fmt.put!("\"你\" == '你': %t\n", result1)
fmt.put!("'你' == \"你\": %t\n", result2)