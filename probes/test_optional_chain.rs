#!/usr/bin/env rustc
// Test: Optional chaining with .?

struct Person {
    name: String,
    age: u32,
}

fn main() {
    // Basic optional field access
    let some_person: Option<Person> = Some(Person {
        name: "Alice".to_string(),
        age: 30,
    });
    let none_person: Option<Person> = None;

    // Test: Some case - should return Some("Alice")
    let name = some_person.?name;
    assert_eq!(name, Some("Alice".to_string()));

    // Test: None case - should return None
    let name = none_person.?name;
    assert_eq!(name, None::<String>);

    // Test: Optional method call
    let opt_string: Option<String> = Some("hello".to_string());
    let upper = opt_string.?to_uppercase();
    assert_eq!(upper, Some("HELLO".to_string()));

    // Test: None method call
    let none_string: Option<String> = None;
    let upper = none_string.?to_uppercase();
    assert_eq!(upper, None::<String>);

    // Test: Method with arguments
    let opt_vec: Option<Vec<i32>> = Some(vec![1, 2, 3]);
    let len = opt_vec.?len();
    assert_eq!(len, Some(3));

    // Test: accessing numeric field (tuple struct)
    let opt_tuple: Option<(i32, String)> = Some((42, "test".to_string()));
    let num = opt_tuple.?0;
    assert_eq!(num, Some(42));

    println!("All optional chaining tests passed!");
}
