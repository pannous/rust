#!/usr/bin/env rust
#![allow(mixed_script_confusables)]
// import math
// Test Unicode identifiers
let δ = 42
let 变量 = "Chinese variable 变量"
// eq!( 变量#-1 , '量');
let переменная = "Russian variable"
let π_custom = 3.14159  // π is now a built-in constant

fn 函数() string {
    return "Unicode function name"
}

fn αβγ() int {
	let δ = 42
    return δ + 10
}

put!("Testing Unicode identifiers:")
put!("δ = %d\n", δ)
put!("变量 = %s\n", 变量)
put!("переменная = %s\n", переменная)
put!("π_custom = %f\n", π_custom)
put!("函数() = %s\n", 函数())
put!("αβγ() = %d\n", αβγ())
put!("All Unicode identifiers work correctly!\n")