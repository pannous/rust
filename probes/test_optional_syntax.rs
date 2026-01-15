#!/usr/bin/env -S cargo +stage1 run --release --manifest-path /opt/other/rust/probes/Cargo.toml --
// Test: T? syntax for Option<T>

fn get_value(flag: bool) -> i32? {
    if flag { Some(42) } else { None }
}

fn process(x: i32?, y: String?) -> bool? {
    let val = x?;
    let s = y?;
    Some(val > 0 && !s.is_empty())
}

struct Config {
    name: String?,
    port: u16?,
}

fn main() {
    // Basic usage
    let _a: i32? = Some(5);
    let _b: i32? = None;

    // Function return type
    let result = get_value(true);
    assert_eq!(result, Some(42));

    let none_result = get_value(false);
    assert_eq!(none_result, None);

    // Multiple optional params
    let processed = process(Some(10), Some("hello".to_string()));
    assert_eq!(processed, Some(true));

    // Struct fields
    let cfg = Config {
        name: Some("test".to_string()),
        port: None,
    };
    assert!(cfg.name.is_some());
    assert!(cfg.port.is_none());

    // Nested optionals: i32?? -> Option<Option<i32>>
    let nested: i32?? = Some(Some(99));
    assert_eq!(nested.flatten(), Some(99));

    println!("All optional syntax tests passed!");
}
