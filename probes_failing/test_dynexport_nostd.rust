#!/usr/bin/env rust
#![no_std]
#![feature(lang_items, start)]

#[dynexport]
#[no_mangle]
pub static VERSION: [u8; 4] = [1, 0, 0, 0];

#[dynexport]
#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[panic_handler]
fn panic!(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
