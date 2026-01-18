#!/usr/bin/env rust
# Test nil as alias for None

# Test nil assignment
let opt: Option<i32> = nil
if opt == None { put!("✓ nil == None") } else { put!("✗ nil != None"); exit(1) }
if opt == nil { put!("✓ nil == nil") } else { put!("✗ nil != nil"); exit(1) }

# Test with explicit type
let typed: Option<i32> = nil
if typed.is_none() { put!("✓ typed nil is_none()") } else { put!("✗ typed nil not is_none()"); exit(1) }

# Test Some value compared to nil
opt2 := Some(42)
if opt2 != nil { put!("✓ Some(42) != nil") } else { put!("✗ Some(42) == nil"); exit(1) }

# Test return nil from function
fn get_opt() -> Option<i32> {
	return nil
}

result := get_opt()
if result == nil { put!("✓ function returned nil") } else { put!("✗ function did not return nil"); exit(1) }

put!("All nil tests passed!")
