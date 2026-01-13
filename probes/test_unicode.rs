#!/usr/bin/env rust
// import math
// Test Unicode identifiers
var δ = 42
var 变量 = "Chinese variable 变量"
#eq!( 变量#-1 , '量');
var переменная = "Russian variable"
var π = 3.14159

func 函数() string {
    return "Unicode function name"
}

func αβγ() int {
    return δ + 10
}

println("Testing Unicode identifiers:")
put!("δ = %d\n", δ)
put!("变量 = %s\n", 变量)
put!("переменная = %s\n", переменная)
put!("π = %f\n", π)
put!("函数() = %s\n", 函数())
put!("αβγ() = %d\n", αβγ())
put!("All Unicode identifiers work correctly!\n")