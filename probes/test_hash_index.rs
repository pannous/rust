#!/usr/bin/env rust

// Test arrays with 1-indexed access
z := ['a', 'b', 'c']
nums := [10, 20, 30, 40]

// Basic 1-indexed access
assert_eq!(z#1, 'a')
assert_eq!(z#2, 'b')
assert_eq!(z#3, 'c')
assert_eq!(nums#1, 10)

// Compare with 0-indexed
assert_eq!(z[0], z#1)
assert_eq!(z[1], z#2)
assert_eq!(z[2], z#3)

// Test with comparison operators (precedence)
assert!(z#1 == 'a')
assert!(z#1 != 'b')
assert!(nums#1 < 15)
assert!(nums#2 > 15)
assert!(nums#1 <= 10)
assert!(nums#2 >= 20)

// Test with arithmetic operators (# should have higher precedence)
assert_eq!(nums#1 + 5, 15)
assert_eq!(nums#2 - 5, 15)
assert_eq!(nums#1 * 2, 20)
assert_eq!(nums#2 / 2, 10)

// Test with logical operators
assert!((z#1 == 'a') && (z#2 == 'b'))
assert!((z#1 == 'a') || (z#2 == 'x'))
assert!(!(z#1 == 'x'))

// Test with expressions as index
idx := 2
assert_eq!(z#idx, 'b')
assert_eq!(z#(1+1), 'b')

// Test with parentheses
assert_eq!((z#1), 'a')
assert_eq!(z#(1), 'a')

// Note: Assignment like `z#1 = 'X'` works in functions but not at script top-level

// Chained access
matrix := [[1, 2, 3], [4, 5, 6]]
assert_eq!(matrix#1#1, 1)
assert_eq!(matrix#1#2, 2)
assert_eq!(matrix#2#1, 4)
assert_eq!(matrix#2#3, 6)

// Mixed with regular indexing
assert_eq!(matrix#1[1], 2)
assert_eq!(matrix[0]#2, 2)

put!("All hash index tests passed!")