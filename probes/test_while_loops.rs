#!/usr/bin/env rust
// Test while loops as synonym for for loops

put!("Testing while loops:\n")

// Basic while loop with condition
i := 0
while i < 3 {
    put!("Basic while: i = %d\n", i)
    i++
}

// While loop with break
j := 0
while true {
    put!("While true: j = %d\n", j)
    j++
    if j >= 3 {
        break
    }
}

// While with for-in syntax for slices
numbers := [1, 2, 3]
put!("While with slice:\n")
while num in numbers {
    put!("  num = %d\n", num)
}

// While with for-in syntax for maps
myMap := map[string]int{"a": 1, "b": 2, "c": 3}
put!("While with map keys:\n")
while key in myMap {
    put!("  key = %s\n", key)
}

// While with key-value pairs
put!("While with key-value pairs:\n")
while key, value in myMap {
    put!("  %s = %d\n", key, value)
}

put!("All while loop tests completed!\n")