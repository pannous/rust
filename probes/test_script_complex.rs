#!/usr/bin/env rust
// Test script with function + macro calls
fn helper() -> i32 { 42 }

eq!(helper(), 42);
println!("Script works!");
