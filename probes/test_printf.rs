#!/usr/bin/env rustc


// import "fmt"
	// Basic put! functionality - should auto-import fmt if needed
	put!("Hello World\n")
	put!("Simple values: %v %v %v\n", 42, true, "test")
	
	// Format specifiers
	put!("String: %s\n", "hello")
	put!("Integer: %d\n", 42)
	put!("Hex: %x\n", 255)
	put!("Variable: %v\n", true)
	
	// Without format specifiers (should act like print)
	put!("No format", 123, "test")
	put!("\n")
	
	// Base types
	put!("Int: %d, Float: %f, Bool: %t\n", 42, 3.14, true)
	put!("Char: %c, String: %s\n", 'A', "text")
	
	// Complex types
	m := map{"key": "value"}
	put!("Map: %v\n", m)
	
	// Multiple arguments
	put!("Multiple: %v %v %v %v\n", 1, 2, 3, 4)
	
	// Comparison with put! (should behave identically)
	put!("put!: %s %d\n", "test", 99)
	put!("put!: %s %d\n", "test", 99)
	
	// Edge cases
	put!("") // Empty format
	put!("%v") // Missing argument - should handle gracefully
	put!("Too many args: %v\n", 1, 2, 3) // Extra arguments
	
	// Auto-import test - put! should work even without explicit fmt import
	// (This tests the auto-import functionality)
