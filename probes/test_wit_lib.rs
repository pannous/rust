#!/usr/bin/env rustc
//! Library with embedded WIT type metadata
//!
//! Compile with:
//!   rustc --edition 2021 --crate-type cdylib test_wit_lib.rs -o libwit_test.dylib

use std::ffi::{c_char, CStr, CString};

/// Embedded WIT type definitions - readable at runtime via dlsym("WIT_TYPES")
#[no_mangle]
pub static WIT_TYPES: &str = r#"
package example:math@1.0.0;

interface math {
    add: fn(a: s32, b: s32) -> s32;
    multiply: fn(a: f64, b: f64) -> f64;
    factorial: fn(n: u32) -> u64;
}

interface strings {
    record person {
        name: string,
        age: u32,
    }

    greet: fn(name: string) -> string;
    greet-person: fn(p: person) -> string;
}

interface arrays {
    sum-array: fn(data: list<s32>) -> s64;
    mean: fn(data: list<f64>) -> f64;
}
"#;

/// WIT type metadata length (useful for reading)
#[no_mangle]
pub static WIT_TYPES_LEN: usize = WIT_TYPES.len();

// =============================================================================
// Math interface implementations
// =============================================================================

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

#[no_mangle]
pub extern "C" fn factorial(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n as u64 * factorial(n - 1),
    }
}

// =============================================================================
// Strings interface implementations
// =============================================================================

/// FFI-safe person struct matching WIT record
#[repr(C)]
pub struct Person {
    pub name: *const c_char,
    pub age: u32,
}

#[no_mangle]
pub extern "C" fn greet(name: *const c_char) -> *mut c_char {
    let name_str = if name.is_null() {
        "World"
    } else {
        unsafe { CStr::from_ptr(name).to_str().unwrap_or("World") }
    };
    let greeting = format!("Hello, {}!", name_str);
    CString::new(greeting).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn greet_person(p: Person) -> *mut c_char {
    let name_str = if p.name.is_null() {
        "Unknown"
    } else {
        unsafe { CStr::from_ptr(p.name).to_str().unwrap_or("Unknown") }
    };
    let greeting = format!("Hello, {} (age {})!", name_str, p.age);
    CString::new(greeting).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn free_string(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}

// =============================================================================
// Arrays interface implementations
// =============================================================================

#[no_mangle]
pub unsafe extern "C" fn sum_array(data: *const i32, len: usize) -> i64 {
    if data.is_null() || len == 0 {
        return 0;
    }
    std::slice::from_raw_parts(data, len)
        .iter()
        .map(|&x| x as i64)
        .sum()
}

#[no_mangle]
pub unsafe extern "C" fn mean(data: *const f64, len: usize) -> f64 {
    if data.is_null() || len == 0 {
        return 0.0;
    }
    let slice = std::slice::from_raw_parts(data, len);
    slice.iter().sum::<f64>() / len as f64
}
