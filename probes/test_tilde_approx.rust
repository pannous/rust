#!/usr/bin/env rust
// Test ~ as synonym for ≈ (approximate equality)

c := 0.1 + 0.2
d := 0.3
put!("(0.1 + 0.2) ~ 0.3:", c ~ d)
assert!(c ~ d)

a := 1.00000000001
b := 1.00000000002
put!("near-identical floats:", a ~ b)
assert!(a ~ b)

// Test ~ with implicit multiplication
put!("τ ~ 2.0π:", τ ~ 2.0π)
assert!(τ ~ 2.0π)

put!("~ works as ≈")
