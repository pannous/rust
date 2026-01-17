#!/usr/bin/env rust

// Test None/Option (Rust equivalent of nil)

opt := None::<i32>
if opt == None {
	put!("Option is None")
}

// Test Some value
opt2 := Some(42)
if opt2 != None {
	put!("Some(42) is not None")
}

// Test return
fn get_opt() -> Option<i32> {
	return None
}

result := get_opt()
if result == None {
	put!("function returned None")
}

put!("All nil tests passed!")
