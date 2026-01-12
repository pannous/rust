#!/usr/bin/env rustc
// Test bare import syntax: import identifier => import "identifier"
import fmt
import strings

func main() {
    // Test that bare imports work correctly
    result := strings.ToUpper("hello world")
    fmt.put!("Result: %s\n", result)
    
    // Test multiple bare imports in same file
    lower := strings.ToLower("GOODBYE WORLD")
    fmt.put!("Lower: %s\n", lower)
}