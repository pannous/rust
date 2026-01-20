#!/usr/bin/env rust
// import "slices"

numbers := @[3, 1, 4, 1, 5]
put!("Original:", numbers)

// Test modifying methods with ! syntax
numbers.sort!()  // should modify in-place
put!("After sort!():", numbers)

numbers.reverse!()  // should modify in-place  
put!("After reverse!():", numbers)

// Test non-modifying methods (return new slice)
sorted := @[5, 2, 8, 1].sorted()  // should return new sorted slice
put!("Sorted (new slice):", sorted)

reversed := @[1, 2, 3, 4].reversed()  // should return new reversed slice  
put!("Reversed (new slice):", reversed)