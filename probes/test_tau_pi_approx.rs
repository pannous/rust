#!/usr/bin/env rust
// Test τ ≈ 2π equality
// import "math"

put!("τ ≈ 2π:", τ ≈ 2*π)
put!("τ value:", τ)
put!("2π value:", 2*π)

// Test some basic approximate equalities
a := 3.14159
b := 3.14160
put!("3.14159 ≈ 3.14160:", a ≈ b)

c := 0.1 + 0.2
d := 0.3
put!("(0.1 + 0.2) ≈ 0.3:", c ≈ d)
put!("(0.1 + 0.2) == 0.3:", c == d)