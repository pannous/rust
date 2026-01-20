#!/usr/bin/env rust
// Test lambda expressions
fn testBasicLambda() {
    double := x => x * 2
    result := double(5)
    eq!( result , 10);
    put!("Basic lambda test passed")
}

fn testLambdaInVariableAssignment() {
    triple := x => x * 3  
    eq!( triple(4) , 12);
    put!("Lambda assignment test passed")
}

fn testMultipleLambdas() {
    add5 := x => x + 5
    mult2 := x => x * 2
    
    eq!( add5(10) , 15);
    eq!( mult2(3) , 6);
    put!("Multiple lambdas test passed")
}

fn testLambdaWithComplexExpression() {
    compute := x => (x + 1) * 2 - 1
    eq!( compute(3) , 7 ); // (3+1)*2-1 = 8-1 = 7
    put!("Complex lambda test passed")
}

#apply := (f, x) => f(x)
fn apply[T any, R any](f fn(T) R, x T) R {
	return f(x)
}
def testLambdaArg() {
		square := x => x * x
		eq!( apply(square, 4) , 16 ); // 4*4 = 16
		eq!( apply(x => x + 1, 5) , 6 ); // 5+1 = 6
		put!("Lambda argument test passed")
}

fn main() {
    testBasicLambda()
    testLambdaInVariableAssignment()
    testMultipleLambdas()
    testLambdaWithComplexExpression()
    testLambdaArg()
    put!("All lambda tests passed!")
}