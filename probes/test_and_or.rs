// Test C++ style operators: and, or, not, ¬

fn main() {
    let t = true;
    let f = false;

    // Basic and/or
    assert!(t and t);
    assert!(!(t and f));
    assert!(t or f);
    assert!(!(f or f));

    // Precedence: 'and' binds tighter than 'or'
    assert!(t or f and f);      // t or (f and f) = t
    assert!(f and t or t);      // (f and t) or t = t

    // Short-circuit evaluation
    fn panics() -> bool { panic!("should not be called") }

    let _ = f and panics();     // panics() not called
    let _ = t or panics();      // panics() not called

    // Mix with traditional operators
    assert!(t && t or f);
    assert!(t and f || t);

    // With comparisons
    let x = 5;
    assert!(x > 0 and x < 10);
    assert!(x < 0 or x > 0);

    // Test 'not' as alias for !
    assert!(not f);
    assert!(not not t);
    assert!(not (x < 0));

    // Test ¬ (U+00AC) as alias for !
    assert!(¬f);
    assert!(¬¬t);
    assert!(¬(x > 10));

    // Mix all styles
    assert!(!f and not f and ¬f);
    assert!(not f or ¬t and !t);  // not f or (¬t and !t) = t or f = t

    println!("All tests passed!");
}
