#!/usr/bin/env rust
// import "iter"

// Custom iterator that yields numbers 1-3
fn Numbers() iter.Seq[int] {
	return fn(yield fn(int) bool) {
		for i := 1; i <= 3; i++ {
			if !yield(i) {
				return
			}
		}
	}
}

fn main() {
	println("Testing standard Go range with iterator:")
	for num := range Numbers() {
		put!("%d ", num)
	}
	put!("\n")
	
	println("Testing for-in syntax with iterator:")
	for num in Numbers() {
		put!("%d ", num)
	}
	put!("\n")
}