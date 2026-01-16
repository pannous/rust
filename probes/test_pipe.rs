#!/usr/bin/env rust
fn square(x int) int {
	return x * x
}

fn double(x int) int {
	return x * 2
}

fn increment(x int) int {
	return x + 1
}

fn negate(x int) int {
	return -x
}

// Basic pipe operator tests
result := 2 | square
println("2 | square =", result)
eq!( result , 4);

// Basic pipe test with different number
result1 := 3 | square
println("3 | square =", result1)
eq!( result1 , 9);

// Pipe with variable
val := 4
result2 := val | double
println("4 | double =", result2)
eq!( result2 , 8);

// Chained pipes
result3 := 2 | square | double
println("2 | square | double =", result3)
eq!( result3 , 8);

// Pipe in assignment
x := 5 | square
println("x = 5 | square =", x)
eq!( x , 25);

// Test in function call arguments
println("Testing pipe in function args:", increment(5 | negate))

// Test with expressions
exprResult := (2 + 3) | increment
println("(2 + 3) | increment =", exprResult)
eq!( exprResult , 6);

// Pipe vs bitwise OR distinction
// This should be pipe operator (function call)
pipeResult := 2 | square
println("2 | square =", pipeResult) // Should be 4
eq!( pipeResult , 4);

// This should be bitwise OR (number | number)
bitwiseResult := 2 | 4
println("2 | 4 =", bitwiseResult) // Should be 6 (bitwise OR)
eq!( bitwiseResult , 6);

// Mixed operations
temp := 3 | square
mixed := temp | 1
println("temp = 3 | square =", temp)
println("temp | 1 =", mixed) // Should be 9 | 1 = 9
eq!( temp , 9);
eq!( mixed , 9);

// Test parenthesized pipe operations
parenResult1 := (3 | square) | 1
println("(3 | square) | 1 =", parenResult1)
eq!( parenResult1 , 9  ); // 3^2 = 9, 9|1 = 9

parenResult2 := 2 | (4 | square)
println("2 | (4 | square) =", parenResult2)
eq!( parenResult2 , 18  ); // 4^2 = 16, 2|16 = 18

println("All pipe operator tests passed!")