//! Test dynload with dynexport_prelude
//!
//! Compile and run with:
//! rustc --edition 2021 test_dynload_prelude.rs -o test_dynload_prelude && ./test_dynload_prelude

use std::ffi::{c_char, c_int, c_void, CStr, CString, OsStr};

// Type definitions matching the prelude
type VecU8Handle = *mut ();
type VecI32Handle = *mut ();
type StringHandle = *mut ();
type HashMapSSHandle = *mut ();

#[repr(C)]
struct OptionI32 {
    value: i32,
    is_some: bool,
}

// Declare dlopen/dlsym/dlclose directly
const RTLD_NOW: c_int = 0x2;

extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> c_int;
    fn dlerror() -> *mut c_char;
}

mod dynload {
    use super::*;

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct DynexportMeta {
        pub type_hash: u64,
        pub compiler_hash: u32,
        pub flags: u32,
    }

    pub struct DynLibrary {
        handle: *mut c_void,
    }

    impl DynLibrary {
        pub unsafe fn open<P: AsRef<OsStr>>(path: P) -> Result<Self, String> {
            let path = path.as_ref();
            let path_cstr = CString::new(path.to_str().unwrap()).unwrap();
            let handle = dlopen(path_cstr.as_ptr(), RTLD_NOW);
            if handle.is_null() {
                let err = dlerror();
                let msg = if err.is_null() {
                    "unknown error".to_string()
                } else {
                    CStr::from_ptr(err).to_string_lossy().to_string()
                };
                return Err(format!("dlopen failed: {}", msg));
            }
            Ok(Self { handle })
        }

        pub fn get_metadata(&self, symbol_name: &str) -> Option<DynexportMeta> {
            let meta_name = format!("dynexport_meta_{}\0", symbol_name);
            unsafe {
                let sym = dlsym(self.handle, meta_name.as_ptr() as *const c_char);
                if sym.is_null() {
                    None
                } else {
                    Some(*(sym as *const DynexportMeta))
                }
            }
        }

        pub unsafe fn get_symbol<T>(&self, name: &str) -> Option<T> {
            let name_cstr = CString::new(name).unwrap();
            let sym = dlsym(self.handle, name_cstr.as_ptr());
            if sym.is_null() {
                None
            } else {
                Some(std::mem::transmute_copy(&sym))
            }
        }
    }

    impl Drop for DynLibrary {
        fn drop(&mut self) {
            unsafe {
                dlclose(self.handle);
            }
        }
    }
}

fn main() {
    println!("=== Testing dynload with dynexport_prelude ===\n");

    // Open the library
    let lib = unsafe {
        dynload::DynLibrary::open("/tmp/libdynexport_prelude_forked.dylib")
            .expect("Failed to open library")
    };

    // Test 1: assert!()metadata for vec_u8_new
    println!("--- Metadata Checks ---");
    if let Some(meta) = lib.get_metadata("vec_u8_new") {
        println!("vec_u8_new metadata:");
        println!("  type_hash: 0x{:016x}", meta.type_hash);
        println!("  compiler_hash: 0x{:08x}", meta.compiler_hash);
        println!("  flags: {}", meta.flags);
    } else {
        println!("WARNING: No metadata for vec_u8_new");
    }

    if let Some(meta) = lib.get_metadata("string_from_cstr") {
        println!("string_from_cstr metadata: type_hash=0x{:016x}", meta.type_hash);
    }

    if let Some(meta) = lib.get_metadata("hashmap_ss_new") {
        println!("hashmap_ss_new metadata: type_hash=0x{:016x}", meta.type_hash);
    }

    // Test 2: Vec<u8> operations
    println!("\n--- Vec<u8> Test ---");
    unsafe {
        let vec_u8_new: fn() -> VecU8Handle = lib.get_symbol("vec_u8_new").unwrap();
        let vec_u8_push: fn(VecU8Handle, u8) = lib.get_symbol("vec_u8_push").unwrap();
        let vec_u8_len: fn(VecU8Handle) -> usize = lib.get_symbol("vec_u8_len").unwrap();
        let vec_u8_get: fn(VecU8Handle, usize) -> u8 = lib.get_symbol("vec_u8_get").unwrap();
        let vec_u8_drop: fn(VecU8Handle) = lib.get_symbol("vec_u8_drop").unwrap();

        let vec = vec_u8_new();
        vec_u8_push(vec, 10);
        vec_u8_push(vec, 20);
        vec_u8_push(vec, 30);

        let len = vec_u8_len(vec);
        println!("Vec<u8> length: {}", len);
        eq!(len, 3);

        let v0 = vec_u8_get(vec, 0);
        let v1 = vec_u8_get(vec, 1);
        let v2 = vec_u8_get(vec, 2);
        println!("Elements: {}, {}, {}", v0, v1, v2);
        eq!(v0, 10);
        eq!(v1, 20);
        eq!(v2, 30);

        vec_u8_drop(vec);
        println!("Vec<u8> test PASSED");
    }

    // Test 3: Vec<i32> operations
    println!("\n--- Vec<i32> Test ---");
    unsafe {
        let vec_i32_new: fn() -> VecI32Handle = lib.get_symbol("vec_i32_new").unwrap();
        let vec_i32_push: fn(VecI32Handle, i32) = lib.get_symbol("vec_i32_push").unwrap();
        let vec_i32_len: fn(VecI32Handle) -> usize = lib.get_symbol("vec_i32_len").unwrap();
        let vec_i32_get: fn(VecI32Handle, usize) -> i32 = lib.get_symbol("vec_i32_get").unwrap();
        let vec_i32_drop: fn(VecI32Handle) = lib.get_symbol("vec_i32_drop").unwrap();

        let vec = vec_i32_new();
        vec_i32_push(vec, -100);
        vec_i32_push(vec, 0);
        vec_i32_push(vec, 100);
        vec_i32_push(vec, 42);

        eq!(vec_i32_len(vec), 4);
        eq!(vec_i32_get(vec, 0), -100);
        eq!(vec_i32_get(vec, 3), 42);

        vec_i32_drop(vec);
        println!("Vec<i32> test PASSED");
    }

    // Test 4: String operations
    println!("\n--- String Test ---");
    unsafe {
        let string_from_cstr: fn(*const c_char) -> StringHandle =
            lib.get_symbol("string_from_cstr").unwrap();
        let string_len: fn(StringHandle) -> usize = lib.get_symbol("string_len").unwrap();
        let string_to_cstr: fn(StringHandle) -> *mut c_char =
            lib.get_symbol("string_to_cstr").unwrap();
        let string_free_cstr: fn(*mut c_char) = lib.get_symbol("string_free_cstr").unwrap();
        let string_push_str: fn(StringHandle, *const c_char) =
            lib.get_symbol("string_push_str").unwrap();
        let string_drop: fn(StringHandle) = lib.get_symbol("string_drop").unwrap();

        let hello = CString::new("Hello").unwrap();
        let s = string_from_cstr(hello.as_ptr());

        eq!(string_len(s), 5);

        let world = CString::new(", World!").unwrap();
        string_push_str(s, world.as_ptr());

        eq!(string_len(s), 13);

        let result = string_to_cstr(s);
        let result_str = CStr::from_ptr(result).to_str().unwrap();
        println!("String content: {}", result_str);
        eq!(result_str, "Hello, World!");

        string_free_cstr(result);
        string_drop(s);
        println!("String test PASSED");
    }

    // Test 5: HashMap<String, String> operations
    println!("\n--- HashMap<String, String> Test ---");
    unsafe {
        let hashmap_ss_new: fn() -> HashMapSSHandle = lib.get_symbol("hashmap_ss_new").unwrap();
        let hashmap_ss_insert: fn(HashMapSSHandle, *const c_char, *const c_char) -> bool =
            lib.get_symbol("hashmap_ss_insert").unwrap();
        let hashmap_ss_get: fn(HashMapSSHandle, *const c_char) -> StringHandle =
            lib.get_symbol("hashmap_ss_get").unwrap();
        let hashmap_ss_len: fn(HashMapSSHandle) -> usize = lib.get_symbol("hashmap_ss_len").unwrap();
        let hashmap_ss_drop: fn(HashMapSSHandle) = lib.get_symbol("hashmap_ss_drop").unwrap();

        let string_to_cstr: fn(StringHandle) -> *mut c_char =
            lib.get_symbol("string_to_cstr").unwrap();
        let string_free_cstr: fn(*mut c_char) = lib.get_symbol("string_free_cstr").unwrap();
        let string_drop: fn(StringHandle) = lib.get_symbol("string_drop").unwrap();

        let map = hashmap_ss_new();

        let k1 = CString::new("name").unwrap();
        let v1 = CString::new("Rust").unwrap();
        assert!(hashmap_ss_insert(map, k1.as_ptr(), v1.as_ptr()));

        let k2 = CString::new("version").unwrap();
        let v2 = CString::new("1.75.0").unwrap();
        hashmap_ss_insert(map, k2.as_ptr(), v2.as_ptr());

        eq!(hashmap_ss_len(map), 2);

        let val = hashmap_ss_get(map, k1.as_ptr());
        assert!(!val.is_null());
        let val_cstr = string_to_cstr(val);
        let val_str = CStr::from_ptr(val_cstr).to_str().unwrap();
        println!("map[\"name\"] = {}", val_str);
        eq!(val_str, "Rust");
        string_free_cstr(val_cstr);
        string_drop(val);

        hashmap_ss_drop(map);
        println!("HashMap<String, String> test PASSED");
    }

    // Test 6: Slice utilities
    println!("\n--- Slice Utilities Test ---");
    unsafe {
        let slice_i32_sum: fn(*const i32, usize) -> i64 =
            lib.get_symbol("slice_i32_sum").unwrap();
        let slice_i32_sort: fn(*mut i32, usize) =
            lib.get_symbol("slice_i32_sort").unwrap();
        let slice_f64_mean: fn(*const f64, usize) -> f64 =
            lib.get_symbol("slice_f64_mean").unwrap();

        // Test sum
        let nums: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let sum = slice_i32_sum(nums.as_ptr(), nums.len());
        println!("Sum of 1..10: {}", sum);
        eq!(sum, 55);

        // Test sort
        let mut unsorted: [i32; 5] = [5, 2, 8, 1, 9];
        slice_i32_sort(unsorted.as_mut_ptr(), unsorted.len());
        println!("Sorted: {:?}", unsorted);
        eq!(unsorted, [1, 2, 5, 8, 9]);

        // Test mean
        let floats: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
        let mean = slice_f64_mean(floats.as_ptr(), floats.len());
        println!("Mean of [1,2,3,4,5]: {}", mean);
        assert!((mean - 3.0).abs() < 0.001);

        println!("Slice utilities test PASSED");
    }

    // Test 7: Verify same compiler for all symbols
    println!("\n--- Compiler Hash Consistency ---");
    let symbols = ["vec_u8_new", "string_from_cstr", "hashmap_ss_new", "slice_i32_sum"];
    let mut hashes: Vec<u32> = Vec::new();
    for sym in &symbols {
        if let Some(meta) = lib.get_metadata(sym) {
            hashes.push(meta.compiler_hash);
        }
    }
    if hashes.len() == symbols.len() {
        let all_same = hashes.iter().all(|&h| h == hashes[0]);
        println!("All {} symbols have same compiler hash: {}", symbols.len(), all_same);
        if all_same {
            println!("Compiler hash: 0x{:08x}", hashes[0]);
        }
    }

    println!("\n=== All tests PASSED! ===");
}
