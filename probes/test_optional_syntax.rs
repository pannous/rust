#!/usr/bin/env rust
// Test: T? syntax for Option<T>

fn get_value(flag: bool) -> i32? {
    if flag { Some(42) } else { None }
}

// Function that auto-wraps its return
fn auto_return(x: i32) -> i32? {
    x + 10  // Auto-wrapped to Some(x + 10)
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

// Basic usage with explicit Some/None
let _a: i32? = Some(5);
let _b: i32? = None;

// Automatic wrapping: T coerces to Option<T>
let auto_wrap: i32? = 42;  // Automatically becomes Some(42)
assert_eq!(auto_wrap, Some(42));

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

// Auto-wrap in function return
let auto_ret = auto_return(5);
assert_eq!(auto_ret, Some(15));

// Auto-wrap in struct field initialization
let cfg_auto = Config {
    name: "auto".to_string(),  // Auto-wrapped to Some("auto".to_string())
    port: 8080,                // Auto-wrapped to Some(8080)
};
assert_eq!(cfg_auto.name, Some("auto".to_string()));
assert_eq!(cfg_auto.port, Some(8080));

// Auto-wrap in function arguments
fn takes_optional(x: i32?) -> i32 {
    x.unwrap_or(0)
}
assert_eq!(takes_optional(42), 42);  // 42 auto-wrapped to Some(42)
assert_eq!(takes_optional(None), 0);

println!("All optional syntax tests passed!");
