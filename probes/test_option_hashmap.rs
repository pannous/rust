//! Test HashMap<i32,i32> with OptionI32 return

use std::ffi::{c_char, c_int, c_void, CString, OsStr};

type HashMapIIHandle = *mut ();

#[repr(C)]
#[derive(Debug)]
struct OptionI32 {
    value: i32,
    is_some: bool,
}

const RTLD_NOW: c_int = 0x2;

extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> c_int;
}

unsafe fn get_symbol<T>(handle: *mut c_void, name: &str) -> T {
    let name_cstr = CString::new(name).unwrap();
    let sym = dlsym(handle, name_cstr.as_ptr());
    std::mem::transmute_copy(&sym)
}

fn main() {
    println!("=== Testing HashMap<i32,i32> with OptionI32 ===\n");

    let path = CString::new("/tmp/libdynexport_prelude_forked.dylib").unwrap();
    let handle = unsafe { dlopen(path.as_ptr(), RTLD_NOW) };
    assert!(!handle.is_null(), "Failed to open library");

    unsafe {
        let hashmap_ii_new: fn() -> HashMapIIHandle = get_symbol(handle, "hashmap_ii_new");
        let hashmap_ii_insert: fn(HashMapIIHandle, i32, i32) = get_symbol(handle, "hashmap_ii_insert");
        let hashmap_ii_get: fn(HashMapIIHandle, i32) -> OptionI32 = get_symbol(handle, "hashmap_ii_get");
        let hashmap_ii_len: fn(HashMapIIHandle) -> usize = get_symbol(handle, "hashmap_ii_len");
        let hashmap_ii_drop: fn(HashMapIIHandle) = get_symbol(handle, "hashmap_ii_drop");

        let map = hashmap_ii_new();

        // Insert some key-value pairs
        hashmap_ii_insert(map, 1, 100);
        hashmap_ii_insert(map, 2, 200);
        hashmap_ii_insert(map, 42, 4200);

        println!("Map length: {}", hashmap_ii_len(map));
        eq!(hashmap_ii_len(map), 3);

        // Test get existing key
        let opt1 = hashmap_ii_get(map, 1);
        println!("get(1) = {:?}", opt1);
        assert!(opt1.is_some);
        eq!(opt1.value, 100);

        let opt42 = hashmap_ii_get(map, 42);
        println!("get(42) = {:?}", opt42);
        assert!(opt42.is_some);
        eq!(opt42.value, 4200);

        // Test get non-existing key
        let opt_none = hashmap_ii_get(map, 999);
        println!("get(999) = {:?}", opt_none);
        assert!(!opt_none.is_some);

        hashmap_ii_drop(map);

        // Test option_i32 functions directly
        let option_i32_some: fn(i32) -> OptionI32 = get_symbol(handle, "option_i32_some");
        let option_i32_none: fn() -> OptionI32 = get_symbol(handle, "option_i32_none");
        let option_i32_unwrap_or: fn(OptionI32, i32) -> i32 = get_symbol(handle, "option_i32_unwrap_or");

        let some_val = option_i32_some(42);
        println!("\noption_i32_some(42) = {:?}", some_val);
        assert!(some_val.is_some);
        eq!(some_val.value, 42);

        let none_val = option_i32_none();
        println!("option_i32_none() = {:?}", none_val);
        assert!(!none_val.is_some);

        let unwrapped = option_i32_unwrap_or(some_val, 0);
        println!("unwrap_or(Some(42), 0) = {}", unwrapped);
        eq!(unwrapped, 42);

        let unwrapped_none = option_i32_unwrap_or(none_val, 999);
        println!("unwrap_or(None, 999) = {}", unwrapped_none);
        eq!(unwrapped_none, 999);

        dlclose(handle);
    }

    println!("\n=== All HashMap<i32,i32> tests PASSED! ===");
}
