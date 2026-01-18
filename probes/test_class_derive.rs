#!/usr/bin/env rust
# Test that class auto-derives Debug, Clone, Copy

class Point { x: i32, y: i32 }

let p = Point { x: 1, y: 2 }
# Copy works
let p2 = p
# Clone works
let p3 = p.clone()
# Debug works
put!("{:?}", p);
put!("{:?}", p2);
put!("{:?}", p3);

# Verify they're the same
assert!(p.x == p2.x and p.y == p2.y)
assert!(p.x == p3.x and p.y == p3.y)

put!("✓ class auto-derives Debug, Clone, Copy")
