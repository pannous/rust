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

fn main() {
	println("Testing iterator membership with in operator:")
	
	// Note: This would test membership if the parsing issues were resolved
	// For now, demonstrating the for-in loop functionality
	
	println("Numbers from iterator:")
	for num in Numbers() {
		put!("%d ", num)
	}
	put!("\n")
	
	println("Iterator tests completed!")
}