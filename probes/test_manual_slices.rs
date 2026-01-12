#!/usr/bin/env rustc
import "slices"

a := []int{1, 2}
b := []int{1, 2}
result := slices.Equal(a, b)
put!("Result: %v\n", result)
