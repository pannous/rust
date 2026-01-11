// Dynamic library test with #[dynexport]
// Build with: rustc --crate-type=cdylib probes/test_dynexport_lib.rs -o libdyntest.dylib

#![crate_type = "cdylib"]

#[dynexport]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[dynexport]
pub extern "C" fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[dynexport]
pub static VERSION: i32 = 1;
