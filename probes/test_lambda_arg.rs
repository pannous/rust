#!/usr/bin/env rust
#Apply := (f, x) => f(x)
fn Apply[T any, R any](f fn(T) R, x T) R {
	return f(x)
}
def testLambdaArg() {
		square := x => x * x
		eq!( Apply(square, 4) , 16 ); // 4*4 = 16
		eq!( Apply(x => x + 1, 5) , 6 ); // 5+1 = 6
		println("Lambda argument test passed")
}

testLambdaArg()
println("All lambda tests passed!")
