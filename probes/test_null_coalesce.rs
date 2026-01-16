#!/usr/bin/env rust
// Test: Null coalescing operator ??

    // Basic null coalescing: Some case
    let some_val: Option<i32> = Some(42);
    let result = some_val ?? 0;
    assert_eq!(result, 42);

    // Basic null coalescing: None case
    let none_val: Option<i32> = None;
    let result = none_val ?? 99;
    assert_eq!(result, 99);

    // Chained null coalescing (right associative)
    let a: Option<i32> = None;
    let b: Option<i32> = None;
    let c: Option<i32> = Some(100);
    // a ?? b ?? c ?? 0 == a ?? (b ?? (c ?? 0))
    let result = a ?? b ?? c ?? 0;
    assert_eq!(result, 100);

    // All None
    let result = a ?? b ?? 55;
    assert_eq!(result, 55);

    // With T? syntax
    let opt: i32? = None;
    let val = opt ?? 123;
    assert_eq!(val, 123);

    let opt2: i32? = 456;  // Auto-wrapped to Some(456)
    let val2 = opt2 ?? 0;
    assert_eq!(val2, 456);

    // Test that existing ??: consecutive try operators still work
    fn returns_result() -> Result<Result<i32, ()>, ()> {
        Ok(Ok(10))
    }
    // This should parse as (returns_result()?)? - two try operators
    fn test_double_try() -> Result<i32, ()> {
        let val = returns_result()??;
        Ok(val)
    }
    assert_eq!(test_double_try(), Ok(10));

    // More complex: foo()? ?? bar
    fn get_option() -> Result<Option<i32>, ()> {
        Ok(None)
    }
    fn combined() -> Result<i32, ()> {
        // get_option()? returns Option<i32>, then ?? provides default
        let val = get_option()? ?? 999;
        Ok(val)
    }
    assert_eq!(combined(), Ok(999));

    println!("All null coalescing tests passed!");
