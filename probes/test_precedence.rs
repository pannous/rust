#!/usr/bin/env rustc
fn main() {
    // Test precedence: and should bind tighter than or (like && vs ||)
    let t = true;
    let f = false;
    
    // t or f and f should be t or (f and f) = t or f = t
    assert!(t or f and f);
    
    // f and t or t should be (f and t) or t = f or t = t  
    assert!(f and t or t);
    
    // Comparison operators should work correctly
    let x = 5;
    let y = 10;
    
    if x < y and y > 0 {
        println!("x < y and y > 0");
    }
    
    // Short-circuit evaluation
    fn side_effect() -> bool {
        println!("side effect!");
        true
    }
    
    // This should NOT print "side effect!" because left side is false
    if false and side_effect() {
        println!("unreachable");
    }
    
    // This should NOT print "side effect!" because left side is true
    if true or side_effect() {
        println!("short-circuited");
    }
    
    println!("All tests passed!");
}
