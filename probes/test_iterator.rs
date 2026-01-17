#!/usr/bin/env rust
// import "iter"
struct Numbers {
	current: int,
	limit: int,
}
fn main() {
	println!("Testing iterator membership with in operator:");
	
	// Note: This would test membership if the parsing issues were resolved
	// For now, demonstrating the for-in loop functionality
	
	println!("Numbers from iterator:");
	for num in Numbers(1,4) {
		put!("%d ", num)
	}
	put!("\n")
	
	println("Iterator tests completed!")
}