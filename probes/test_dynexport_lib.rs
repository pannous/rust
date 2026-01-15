#!/usr/bin/env rustc
//! Example: Hello World dynamic library
//!
//! Compile with:
//!   rustc --edition 2021 --crate-type cdylib hello_lib.rs -o libhello.dylib
//!
//! Then run hello_user.rs to load and use it.

use std::ffi::{c_char, CStr, CString};

/// Add two integers - simplest possible export
#[dynexport]
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiply two floats
#[dynexport]
#[no_mangle]
pub extern "C" fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// Create a greeting string (caller must free with free_string)
#[dynexport]
#[no_mangle]
pub extern "C" fn greet(name: *const c_char) -> *mut c_char {
    let name = if name.is_null() {
        "World"
    } else {
        unsafe { CStr::from_ptr(name).to_str().unwrap_or("World") }
    };

    let greeting = format!("Hello, {}!", name);
    CString::new(greeting).unwrap().into_raw()
}

/// Free a string returned by greet()
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn free_string(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}

/// Compute factorial (demonstrates recursion works)
#[dynexport]
#[no_mangle]
pub extern "C" fn factorial(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n as u64 * factorial(n - 1),
    }
}

/// Sum an array of integers
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn sum_array(arr: *const i32, len: usize) -> i64 {
    if arr.is_null() || len == 0 {
        return 0;
    }
    std::slice::from_raw_parts(arr, len)
        .iter()
        .map(|&x| x as i64)
        .sum()
}
