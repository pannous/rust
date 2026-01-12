//! Test: Link against dynexport library at compile time (like C/C++)
//!
//! Compile library first:
//!   rustc --edition 2021 --crate-type cdylib test_dynexport_lib.rs -o libdynexport_test.dylib
//!
//! Then compile this with -L and -l flags:
//!   rustc --edition 2021 -L . -l dynexport_test test_dynexport_linked.rs
//!
//! Run with library path:
//!   DYLD_LIBRARY_PATH=. ./test_dynexport_linked   # macOS
//!   LD_LIBRARY_PATH=. ./test_dynexport_linked     # Linux

use std::ffi::{c_char, CStr, CString};

// Link against the library - just like C's -l flag
#[link(name = "dynexport_test")]
extern "C" {
    fn add(a: i32, b: i32) -> i32;
    fn multiply(a: f64, b: f64) -> f64;
    fn greet(name: *const c_char) -> *mut c_char;
    fn free_string(s: *mut c_char);
    fn factorial(n: u32) -> u64;
    fn sum_array(arr: *const i32, len: usize) -> i64;
}

fn main() {
    println!("=== Linked Dynamic Library Test ===\n");

    // Direct calls - no dlsym needed!
    println!("add(2, 3) = {}", unsafe { add(2, 3) });
    assert_eq!(unsafe { add(2, 3) }, 5);

    println!("multiply(3.14, 2.0) = {:.2}", unsafe { multiply(3.14, 2.0) });
    assert!((unsafe { multiply(3.14, 2.0) } - 6.28).abs() < 0.001);

    println!("factorial(10) = {}", unsafe { factorial(10) });
    assert_eq!(unsafe { factorial(10) }, 3628800);

    let nums = [1i32, 2, 3, 4, 5];
    println!("sum_array([1,2,3,4,5]) = {}", unsafe { sum_array(nums.as_ptr(), nums.len()) });
    assert_eq!(unsafe { sum_array(nums.as_ptr(), nums.len()) }, 15);

    // String handling
    let name = CString::new("Linked Rust").unwrap();
    let greeting = unsafe { greet(name.as_ptr()) };
    let greeting_str = unsafe { CStr::from_ptr(greeting).to_str().unwrap() };
    println!("greet(\"Linked Rust\") = {}", greeting_str);
    assert_eq!(greeting_str, "Hello, Linked Rust!");
    unsafe { free_string(greeting) };

    println!("\n=== All tests passed! ===");
}
