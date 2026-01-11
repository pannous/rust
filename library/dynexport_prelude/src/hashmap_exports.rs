//! HashMap exports for common key-value types.

use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;
use super::string_exports::StringHandle;
use super::option_exports::OptionI32;

// =============================================================================
// HashMap<String, String> - Most common use case
// =============================================================================

pub type HashMapStringStringHandle = *mut HashMap<String, String>;

#[dynexport]
#[no_mangle]
pub extern "C" fn hashmap_string_string_new() -> HashMapStringStringHandle {
    Box::into_raw(Box::new(HashMap::<String, String>::new()))
}

#[dynexport]
#[no_mangle]
pub extern "C" fn hashmap_string_string_with_capacity(capacity: usize) -> HashMapStringStringHandle {
    Box::into_raw(Box::new(HashMap::<String, String>::with_capacity(capacity)))
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_string_string_insert(
    handle: HashMapStringStringHandle,
    key: *const c_char,
    value: *const c_char,
) -> bool {
    if handle.is_null() || key.is_null() || value.is_null() { return false; }
    let key = match CStr::from_ptr(key).to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return false,
    };
    let value = match CStr::from_ptr(value).to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return false,
    };
    (*handle).insert(key, value);
    true
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_string_string_get(
    handle: HashMapStringStringHandle,
    key: *const c_char,
) -> StringHandle {
    if handle.is_null() || key.is_null() { return ptr::null_mut(); }
    let key = match CStr::from_ptr(key).to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    match (*handle).get(key) {
        Some(v) => Box::into_raw(Box::new(v.clone())),
        None => ptr::null_mut(),
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_string_string_contains_key(
    handle: HashMapStringStringHandle,
    key: *const c_char,
) -> bool {
    if handle.is_null() || key.is_null() { return false; }
    let key = match CStr::from_ptr(key).to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };
    (*handle).contains_key(key)
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_string_string_remove(
    handle: HashMapStringStringHandle,
    key: *const c_char,
) -> StringHandle {
    if handle.is_null() || key.is_null() { return ptr::null_mut(); }
    let key = match CStr::from_ptr(key).to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    match (*handle).remove(key) {
        Some(v) => Box::into_raw(Box::new(v)),
        None => ptr::null_mut(),
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_string_string_len(handle: HashMapStringStringHandle) -> usize {
    if handle.is_null() { return 0; }
    (*handle).len()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_string_string_is_empty(handle: HashMapStringStringHandle) -> bool {
    if handle.is_null() { return true; }
    (*handle).is_empty()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_string_string_clear(handle: HashMapStringStringHandle) {
    if !handle.is_null() {
        (*handle).clear();
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_string_string_drop(handle: HashMapStringStringHandle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

// =============================================================================
// HashMap<String, i32>
// =============================================================================

pub type HashMapStringI32Handle = *mut HashMap<String, i32>;

#[dynexport]
#[no_mangle]
pub extern "C" fn hashmap_string_i32_new() -> HashMapStringI32Handle {
    Box::into_raw(Box::new(HashMap::<String, i32>::new()))
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_string_i32_insert(
    handle: HashMapStringI32Handle,
    key: *const c_char,
    value: i32,
) -> bool {
    if handle.is_null() || key.is_null() { return false; }
    let key = match CStr::from_ptr(key).to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return false,
    };
    (*handle).insert(key, value);
    true
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_string_i32_get(
    handle: HashMapStringI32Handle,
    key: *const c_char,
) -> OptionI32 {
    if handle.is_null() || key.is_null() {
        return OptionI32 { value: 0, is_some: false };
    }
    let key = match CStr::from_ptr(key).to_str() {
        Ok(s) => s,
        Err(_) => return OptionI32 { value: 0, is_some: false },
    };
    match (*handle).get(key) {
        Some(&v) => OptionI32 { value: v, is_some: true },
        None => OptionI32 { value: 0, is_some: false },
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_string_i32_len(handle: HashMapStringI32Handle) -> usize {
    if handle.is_null() { return 0; }
    (*handle).len()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_string_i32_drop(handle: HashMapStringI32Handle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

// =============================================================================
// HashMap<i32, i32>
// =============================================================================

pub type HashMapI32I32Handle = *mut HashMap<i32, i32>;

#[dynexport]
#[no_mangle]
pub extern "C" fn hashmap_i32_i32_new() -> HashMapI32I32Handle {
    Box::into_raw(Box::new(HashMap::<i32, i32>::new()))
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_i32_i32_insert(
    handle: HashMapI32I32Handle,
    key: i32,
    value: i32,
) {
    if !handle.is_null() {
        (*handle).insert(key, value);
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_i32_i32_get(handle: HashMapI32I32Handle, key: i32) -> OptionI32 {
    if handle.is_null() {
        return OptionI32 { value: 0, is_some: false };
    }
    match (*handle).get(&key) {
        Some(&v) => OptionI32 { value: v, is_some: true },
        None => OptionI32 { value: 0, is_some: false },
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_i32_i32_contains_key(handle: HashMapI32I32Handle, key: i32) -> bool {
    if handle.is_null() { return false; }
    (*handle).contains_key(&key)
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_i32_i32_len(handle: HashMapI32I32Handle) -> usize {
    if handle.is_null() { return 0; }
    (*handle).len()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_i32_i32_drop(handle: HashMapI32I32Handle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

// =============================================================================
// HashMap<i64, String>
// =============================================================================

pub type HashMapI64StringHandle = *mut HashMap<i64, String>;

#[dynexport]
#[no_mangle]
pub extern "C" fn hashmap_i64_string_new() -> HashMapI64StringHandle {
    Box::into_raw(Box::new(HashMap::<i64, String>::new()))
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_i64_string_insert(
    handle: HashMapI64StringHandle,
    key: i64,
    value: *const c_char,
) -> bool {
    if handle.is_null() || value.is_null() { return false; }
    let value = match CStr::from_ptr(value).to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return false,
    };
    (*handle).insert(key, value);
    true
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_i64_string_get(
    handle: HashMapI64StringHandle,
    key: i64,
) -> StringHandle {
    if handle.is_null() { return ptr::null_mut(); }
    match (*handle).get(&key) {
        Some(v) => Box::into_raw(Box::new(v.clone())),
        None => ptr::null_mut(),
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_i64_string_len(handle: HashMapI64StringHandle) -> usize {
    if handle.is_null() { return 0; }
    (*handle).len()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_i64_string_drop(handle: HashMapI64StringHandle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}
