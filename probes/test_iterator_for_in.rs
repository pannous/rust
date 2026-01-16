#!/usr/bin/env rust
// import "iter"

// Custom iterator that yields numbers 1-5
fn Numbers() iter.Seq[int] {
	return fn(yield fn(int) bool) {
		for i := 1; i <= 5; i++ {
			if !yield(i) {
				return
			}
		}
	}
}

// Custom iterator that yields key-value pairs
fn KeyValuePairs() iter.Seq2[string, int] {
	return fn(yield fn(string, int) bool) {
		pairs := map[string]int{"a": 1, "b": 2, "c": 3}
		for k, v := range pairs {
			if !yield(k, v) {
				return
			}
		}
	}
}

fn main() {
	println("Testing for-in with single value iterator:")
	for num in Numbers() {
		put!("%d ", num)
	}
	put!("\n")

	println("Testing for-in with key-value iterator:")
	for key, value in KeyValuePairs() {
		put!("%s:%d ", key, value)
	}
	put!("\n")

	println("Iterator for-in tests completed!")
}