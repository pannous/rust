#!/usr/bin/env rust
// Test for-in with both single and double variable patterns
myMap := map[string]int{
	"apple":  1,
	"banana": 2,
	"cherry": 3,
}

put!("Single variable (keys only):\n")
for key in myMap {
	put!(key)
}

put!("\nDouble variable (key-value pairs):\n")
for key, value in myMap {
	put!("Key: %s, Value: %d\n", key, value)
}

put!("\nTesting with slice:")
numbers := [10, 20, 30]

put!("Single variable (values):\n")
for num in numbers {
	put!("Value: %d\n", num)
}

put!("\nDouble variable (index-value pairs):\n")
for i, num in numbers {
	put!("Index: %d, Value: %d\n", i, num)
}

put!("All key-value tests completed!")