//! Debug OptionI32 layout

use std::ffi::{c_char, c_int, c_void, CString};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct OptionI32 {
    value: i32,
    is_some: bool,
}

const RTLD_NOW: c_int = 0x2;

extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

fn main() {
    println!("OptionI32 size: {}", std::mem::size_of::<OptionI32>());
    println!("OptionI32 align: {}", std::mem::align_of::<OptionI32>());
    println!("value offset: {}", std::mem::offset_of!(OptionI32, value));
    println!("is_some offset: {}", std::mem::offset_of!(OptionI32, is_some));

    let path = CString::new("/tmp/libdynexport_prelude_forked.dylib").unwrap();
    let handle = unsafe { dlopen(path.as_ptr(), RTLD_NOW) };

    unsafe {
        // Get option_i32_some and call it
        let name = CString::new("option_i32_some").unwrap();
        let sym = dlsym(handle, name.as_ptr());
        let option_i32_some: fn(i32) -> OptionI32 = std::mem::transmute_copy(&sym);

        let result = option_i32_some(42);
        println!("\nCalling option_i32_some(42):");
        println!("  result.value = {}", result.value);
        println!("  result.is_some = {}", result.is_some);

        // Print raw bytes
        let ptr = &result as *const OptionI32 as *const u8;
        print!("  raw bytes: ");
        for i in 0..std::mem::size_of::<OptionI32>() {
            print!("{:02x} ", *ptr.add(i));
        }
        println!();

        // Try reading as u64 to see all bytes
        let raw: u64 = std::mem::transmute_copy(&result);
        println!("  as u64: 0x{:016x}", raw);
    }
}
