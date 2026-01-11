//! Debug OptionI32 ABI with extern "C"

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
    let path = CString::new("/tmp/libdynexport_prelude_forked.dylib").unwrap();
    let handle = unsafe { dlopen(path.as_ptr(), RTLD_NOW) };

    unsafe {
        // Try with extern "C" fn type
        let name = CString::new("option_i32_some").unwrap();
        let sym = dlsym(handle, name.as_ptr());

        // Cast to extern "C" fn
        let option_i32_some: extern "C" fn(i32) -> OptionI32 = std::mem::transmute(sym);

        let result = option_i32_some(42);
        println!("With extern \"C\" fn:");
        println!("  result.value = {}", result.value);
        println!("  result.is_some = {}", result.is_some);

        // Raw bytes
        let ptr = &result as *const OptionI32 as *const u8;
        print!("  raw bytes: ");
        for i in 0..8 {
            print!("{:02x} ", *ptr.add(i));
        }
        println!();

        // Also test hashmap_ii_get which returns OptionI32
        let name = CString::new("hashmap_ii_new").unwrap();
        let sym = dlsym(handle, name.as_ptr());
        let hashmap_ii_new: extern "C" fn() -> *mut () = std::mem::transmute(sym);

        let name = CString::new("hashmap_ii_insert").unwrap();
        let sym = dlsym(handle, name.as_ptr());
        let hashmap_ii_insert: extern "C" fn(*mut (), i32, i32) = std::mem::transmute(sym);

        let name = CString::new("hashmap_ii_get").unwrap();
        let sym = dlsym(handle, name.as_ptr());
        let hashmap_ii_get: extern "C" fn(*mut (), i32) -> OptionI32 = std::mem::transmute(sym);

        let name = CString::new("hashmap_ii_drop").unwrap();
        let sym = dlsym(handle, name.as_ptr());
        let hashmap_ii_drop: extern "C" fn(*mut ()) = std::mem::transmute(sym);

        let map = hashmap_ii_new();
        hashmap_ii_insert(map, 1, 100);

        let opt = hashmap_ii_get(map, 1);
        println!("\nhashmap_ii_get(map, 1):");
        println!("  opt.value = {}", opt.value);
        println!("  opt.is_some = {}", opt.is_some);

        let ptr = &opt as *const OptionI32 as *const u8;
        print!("  raw bytes: ");
        for i in 0..8 {
            print!("{:02x} ", *ptr.add(i));
        }
        println!();

        hashmap_ii_drop(map);
    }
}
