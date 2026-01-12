#!/usr/bin/env rustc
// import "iter"

// Custom iterator that yields numbers 1-5
func Numbers() iter.Seq[int] {
	return func(yield func(int) bool) {
		for i := 1; i <= 5; i++ {
			if !yield(i) {
				return
			}
		}
	}
}

// Custom iterator that yields key-value pairs
func KeyValuePairs() iter.Seq2[string, int] {
	return func(yield func(string, int) bool) {
		pairs := map[string]int{"a": 1, "b": 2, "c": 3}
		for k, v := range pairs {
			if !yield(k, v) {
				return
			}
		}
	}
}

func main() {
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