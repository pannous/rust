// Probes test library for custom Rust extensions
// This module provides common utilities for testing custom syntax features
//
// NOTE: Tests in this crate use custom syntax extensions and must be run
// with the custom rustc compiler via ./run_all_tests.sh
// Standard cargo test will not work due to custom syntax like ':=' operator

#![allow(unused_macros)]
#![allow(dead_code)]

// Common test macros that mirror custom syntax
#[macro_export]
macro_rules! eq {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

#[macro_export]
macro_rules! put {
    ($($arg:tt)*) => {
        println!($($arg)*)
    };
}

// Exit macros
#[macro_export]
macro_rules! exit {
    () => {
        std::process::exit(0)
    };
    ($code:expr) => {
        std::process::exit($code)
    };
}

// Math constants (τ = 2π, π)
pub const TAU: f64 = std::f64::consts::TAU;
pub const PI: f64 = std::f64::consts::PI;

// Type aliases matching custom syntax
pub type int = i32;
pub type float = f64;
pub type boolean = bool;
