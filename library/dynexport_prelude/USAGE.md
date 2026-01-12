# Dynamic Export Quick Start

This guide shows how to create and use dynamically linked Rust libraries with `#[dynexport]`.

## Creating a Library

### 1. Write your library (`mylib.rs`)

```rust
//! A simple dynamically-linked library

#[dynexport]
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[dynexport]
#[no_mangle]
pub extern "C" fn greet(name: *const std::ffi::c_char) -> *mut std::ffi::c_char {
    use std::ffi::{CStr, CString};

    let name = unsafe {
        if name.is_null() { "World" }
        else { CStr::from_ptr(name).to_str().unwrap_or("World") }
    };

    let greeting = format!("Hello, {}!", name);
    CString::new(greeting).unwrap().into_raw()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn free_string(s: *mut std::ffi::c_char) {
    if !s.is_null() {
        drop(std::ffi::CString::from_raw(s));
    }
}
```

### 2. Compile as a dynamic library

```bash
# Using the forked rustc with #[dynexport] support
rustc --edition 2021 --crate-type cdylib mylib.rs -o libmylib.dylib

# On Linux, output would be libmylib.so
# On Windows, output would be mylib.dll
```

### 3. Verify exports

```bash
# Check that symbols and metadata are exported
nm -gU libmylib.dylib | grep -E "(add|greet|dynexport_meta)"

# Expected output:
# ... T _add
# ... T _greet
# ... T _free_string
# ... D _dynexport_meta_add
# ... D _dynexport_meta_greet
# ... D _dynexport_meta_free_string
```

## Using the Library

### From Rust

```rust
use std::ffi::{c_char, c_int, c_void, CStr, CString};

const RTLD_NOW: c_int = 0x2;

extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> c_int;
}

fn main() {
    let path = CString::new("./libmylib.dylib").unwrap();
    let handle = unsafe { dlopen(path.as_ptr(), RTLD_NOW) };
    assert!(!handle.is_null(), "Failed to load library");

    unsafe {
        // Load the add function
        let sym = CString::new("add").unwrap();
        let add: extern "C" fn(i32, i32) -> i32 =
            std::mem::transmute(dlsym(handle, sym.as_ptr()));

        println!("add(2, 3) = {}", add(2, 3));  // Output: 5

        // Load and call greet
        let sym = CString::new("greet").unwrap();
        let greet: extern "C" fn(*const c_char) -> *mut c_char =
            std::mem::transmute(dlsym(handle, sym.as_ptr()));

        let sym = CString::new("free_string").unwrap();
        let free_string: extern "C" fn(*mut c_char) =
            std::mem::transmute(dlsym(handle, sym.as_ptr()));

        let name = CString::new("Rust").unwrap();
        let greeting = greet(name.as_ptr());
        println!("{}", CStr::from_ptr(greeting).to_str().unwrap());  // Output: Hello, Rust!
        free_string(greeting);

        dlclose(handle);
    }
}
```

### From C

```c
#include <stdio.h>
#include <dlfcn.h>

int main() {
    void *handle = dlopen("./libmylib.dylib", RTLD_NOW);
    if (!handle) {
        printf("Error: %s\n", dlerror());
        return 1;
    }

    // Load functions
    int (*add)(int, int) = dlsym(handle, "add");
    char* (*greet)(const char*) = dlsym(handle, "greet");
    void (*free_string)(char*) = dlsym(handle, "free_string");

    // Use them
    printf("add(2, 3) = %d\n", add(2, 3));

    char *msg = greet("C");
    printf("%s\n", msg);
    free_string(msg);

    dlclose(handle);
    return 0;
}
```

Compile and run:
```bash
cc -o test_c test.c -ldl
./test_c
```

## ABI Verification

The `#[dynexport]` attribute automatically generates metadata for each exported symbol:

```rust
#[repr(C)]
struct DynexportMeta {
    type_hash: u64,      // Hash of function signature
    compiler_hash: u32,  // Hash of compiler version
    flags: u32,          // Reserved
}
```

Access metadata to verify ABI compatibility:

```rust
// Metadata symbol name = "dynexport_meta_" + function_name
let meta_sym = CString::new("dynexport_meta_add").unwrap();
let meta: *const DynexportMeta = std::mem::transmute(dlsym(handle, meta_sym.as_ptr()));

if !meta.is_null() {
    let meta = unsafe { *meta };
    println!("type_hash: 0x{:016x}", meta.type_hash);
    println!("compiler_hash: 0x{:08x}", meta.compiler_hash);
}
```

## Using the Prelude

The `dynexport_prelude` provides pre-exported generic types:

| Type | Functions |
|------|-----------|
| `Vec<u8>` | `vec_u8_new`, `vec_u8_push`, `vec_u8_len`, `vec_u8_get`, `vec_u8_drop` |
| `Vec<i32>` | `vec_i32_new`, `vec_i32_push`, `vec_i32_len`, `vec_i32_get`, `vec_i32_drop` |
| `Vec<f64>` | `vec_f64_new`, `vec_f64_push`, `vec_f64_len`, `vec_f64_get`, `vec_f64_drop` |
| `String` | `string_new`, `string_from_cstr`, `string_len`, `string_push_str`, `string_to_cstr`, `string_clone`, `string_drop` |
| `HashMap<String,String>` | `hashmap_ss_new`, `hashmap_ss_insert`, `hashmap_ss_get`, `hashmap_ss_len`, `hashmap_ss_drop` |
| `HashMap<i32,i32>` | `hashmap_ii_new`, `hashmap_ii_insert`, `hashmap_ii_get`, `hashmap_ii_len`, `hashmap_ii_drop` |
| `Option<i32>` | `option_i32_some`, `option_i32_none`, `option_i32_unwrap_or` |
| Slices | `slice_i32_sum`, `slice_i32_sort`, `slice_f64_sum`, `slice_f64_mean` |

Build the prelude:
```bash
rustc --edition 2021 --crate-type cdylib \
    library/dynexport_prelude/src/standalone.rs \
    -o libdynexport_prelude.dylib
```

## Important Notes

1. **Use `extern "C"`** - All exported functions must use `extern "C"` calling convention

2. **Use `#[no_mangle]`** - Prevents Rust name mangling for stable symbol names

3. **Handle types must be opaque pointers** - Return `*mut T` as handles, not `T` directly

4. **Free what you allocate** - Provide `*_drop` or `*_free` functions for heap allocations

5. **Strings need conversion** - Use `CString`/`CStr` for C-compatible strings

6. **Struct returns need `extern "C" fn`** - When loading functions that return structs via `dlsym`, cast to `extern "C" fn`, not plain `fn`
