#!/usr/bin/env rustc
// Test script with function + macro calls
fn helper() -> i32 { 42 }

eq!(helper(), 42);
println!("Script works!");
