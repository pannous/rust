fn main() {
    let t = true;
    let f = false;
    
    // Test 'not' as !
    assert!(not f);
    assert!(not not t);
    
    // Test ¬ as !
    assert!(¬f);
    assert!(¬¬t);
    
    // Mix all styles
    assert!(!f and not f and ¬f);
    
    // With comparisons
    let x = 5;
    assert!(not (x < 0));
    assert!(¬(x > 10));
    
    println!("All not/¬ tests passed!");
}
