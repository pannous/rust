//! Pre-instantiated generic exports for dynamic linking.
//! Single-file version for standalone compilation.

#![allow(clippy::missing_safety_doc)]
#![allow(dangerous_implicit_autorefs)]

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

/// Version of the prelude ABI.
#[dynexport]
#[no_mangle]
pub static DYNEXPORT_PRELUDE_VERSION: u32 = 1;

// =============================================================================
// Vec<u8>
// =============================================================================

pub type VecU8Handle = *mut Vec<u8>;

#[dynexport]
#[no_mangle]
pub extern "C" fn vec_u8_new() -> VecU8Handle {
    Box::into_raw(Box::new(Vec::<u8>::new()))
}

#[dynexport]
#[no_mangle]
pub extern "C" fn vec_u8_with_capacity(capacity: usize) -> VecU8Handle {
    Box::into_raw(Box::new(Vec::<u8>::with_capacity(capacity)))
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_u8_push(handle: VecU8Handle, value: u8) {
    if !handle.is_null() { (*handle).push(value); }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_u8_pop(handle: VecU8Handle) -> u8 {
    if handle.is_null() { return 0; }
    (*handle).pop().unwrap_or(0)
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_u8_len(handle: VecU8Handle) -> usize {
    if handle.is_null() { return 0; }
    (*handle).len()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_u8_get(handle: VecU8Handle, index: usize) -> u8 {
    if handle.is_null() { return 0; }
    (*handle).get(index).copied().unwrap_or(0)
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_u8_as_ptr(handle: VecU8Handle) -> *const u8 {
    if handle.is_null() { return ptr::null(); }
    (*handle).as_ptr()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_u8_drop(handle: VecU8Handle) {
    if !handle.is_null() { drop(Box::from_raw(handle)); }
}

// =============================================================================
// Vec<i32>
// =============================================================================

pub type VecI32Handle = *mut Vec<i32>;

#[dynexport]
#[no_mangle]
pub extern "C" fn vec_i32_new() -> VecI32Handle {
    Box::into_raw(Box::new(Vec::<i32>::new()))
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_i32_push(handle: VecI32Handle, value: i32) {
    if !handle.is_null() { (*handle).push(value); }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_i32_len(handle: VecI32Handle) -> usize {
    if handle.is_null() { return 0; }
    (*handle).len()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_i32_get(handle: VecI32Handle, index: usize) -> i32 {
    if handle.is_null() { return 0; }
    (*handle).get(index).copied().unwrap_or(0)
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_i32_drop(handle: VecI32Handle) {
    if !handle.is_null() { drop(Box::from_raw(handle)); }
}

// =============================================================================
// Vec<f64>
// =============================================================================

pub type VecF64Handle = *mut Vec<f64>;

#[dynexport]
#[no_mangle]
pub extern "C" fn vec_f64_new() -> VecF64Handle {
    Box::into_raw(Box::new(Vec::<f64>::new()))
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_f64_push(handle: VecF64Handle, value: f64) {
    if !handle.is_null() { (*handle).push(value); }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_f64_len(handle: VecF64Handle) -> usize {
    if handle.is_null() { return 0; }
    (*handle).len()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_f64_get(handle: VecF64Handle, index: usize) -> f64 {
    if handle.is_null() { return 0.0; }
    (*handle).get(index).copied().unwrap_or(0.0)
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_f64_drop(handle: VecF64Handle) {
    if !handle.is_null() { drop(Box::from_raw(handle)); }
}

// =============================================================================
// String
// =============================================================================

pub type StringHandle = *mut String;

#[dynexport]
#[no_mangle]
pub extern "C" fn string_new() -> StringHandle {
    Box::into_raw(Box::new(String::new()))
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn string_from_cstr(s: *const c_char) -> StringHandle {
    if s.is_null() { return Box::into_raw(Box::new(String::new())); }
    let cstr = CStr::from_ptr(s);
    match cstr.to_str() {
        Ok(s) => Box::into_raw(Box::new(s.to_string())),
        Err(_) => Box::into_raw(Box::new(String::new())),
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn string_len(handle: StringHandle) -> usize {
    if handle.is_null() { return 0; }
    (*handle).len()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn string_push_str(handle: StringHandle, s: *const c_char) {
    if handle.is_null() || s.is_null() { return; }
    if let Ok(s) = CStr::from_ptr(s).to_str() { (*handle).push_str(s); }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn string_to_cstr(handle: StringHandle) -> *mut c_char {
    if handle.is_null() { return ptr::null_mut(); }
    match CString::new((*handle).as_str()) {
        Ok(cstr) => cstr.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn string_free_cstr(s: *mut c_char) {
    if !s.is_null() { drop(CString::from_raw(s)); }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn string_clone(handle: StringHandle) -> StringHandle {
    if handle.is_null() { return Box::into_raw(Box::new(String::new())); }
    Box::into_raw(Box::new((*handle).clone()))
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn string_drop(handle: StringHandle) {
    if !handle.is_null() { drop(Box::from_raw(handle)); }
}

// =============================================================================
// Vec<String>
// =============================================================================

pub type VecStringHandle = *mut Vec<String>;

#[dynexport]
#[no_mangle]
pub extern "C" fn vec_string_new() -> VecStringHandle {
    Box::into_raw(Box::new(Vec::<String>::new()))
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_string_push_cstr(handle: VecStringHandle, s: *const c_char) {
    if handle.is_null() || s.is_null() { return; }
    if let Ok(s) = CStr::from_ptr(s).to_str() { (*handle).push(s.to_string()); }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_string_len(handle: VecStringHandle) -> usize {
    if handle.is_null() { return 0; }
    (*handle).len()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_string_drop(handle: VecStringHandle) {
    if !handle.is_null() { drop(Box::from_raw(handle)); }
}

// =============================================================================
// Option<i32>
// =============================================================================

#[repr(C)]
pub struct OptionI32 { pub value: i32, pub is_some: bool }

#[dynexport]
#[no_mangle]
pub extern "C" fn option_i32_some(value: i32) -> OptionI32 {
    OptionI32 { value, is_some: true }
}

#[dynexport]
#[no_mangle]
pub extern "C" fn option_i32_none() -> OptionI32 {
    OptionI32 { value: 0, is_some: false }
}

#[dynexport]
#[no_mangle]
pub extern "C" fn option_i32_unwrap_or(opt: OptionI32, default: i32) -> i32 {
    if opt.is_some { opt.value } else { default }
}

// =============================================================================
// HashMap<String, String>
// =============================================================================

pub type HashMapSSHandle = *mut HashMap<String, String>;

#[dynexport]
#[no_mangle]
pub extern "C" fn hashmap_ss_new() -> HashMapSSHandle {
    Box::into_raw(Box::new(HashMap::<String, String>::new()))
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_ss_insert(handle: HashMapSSHandle, key: *const c_char, value: *const c_char) -> bool {
    if handle.is_null() || key.is_null() || value.is_null() { return false; }
    let key = match CStr::from_ptr(key).to_str() { Ok(s) => s.to_string(), Err(_) => return false };
    let value = match CStr::from_ptr(value).to_str() { Ok(s) => s.to_string(), Err(_) => return false };
    (*handle).insert(key, value);
    true
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_ss_get(handle: HashMapSSHandle, key: *const c_char) -> StringHandle {
    if handle.is_null() || key.is_null() { return ptr::null_mut(); }
    let key = match CStr::from_ptr(key).to_str() { Ok(s) => s, Err(_) => return ptr::null_mut() };
    match (*handle).get(key) {
        Some(v) => Box::into_raw(Box::new(v.clone())),
        None => ptr::null_mut(),
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_ss_len(handle: HashMapSSHandle) -> usize {
    if handle.is_null() { return 0; }
    (*handle).len()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_ss_drop(handle: HashMapSSHandle) {
    if !handle.is_null() { drop(Box::from_raw(handle)); }
}

// =============================================================================
// HashMap<i32, i32>
// =============================================================================

pub type HashMapIIHandle = *mut HashMap<i32, i32>;

#[dynexport]
#[no_mangle]
pub extern "C" fn hashmap_ii_new() -> HashMapIIHandle {
    Box::into_raw(Box::new(HashMap::<i32, i32>::new()))
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_ii_insert(handle: HashMapIIHandle, key: i32, value: i32) {
    if !handle.is_null() { (*handle).insert(key, value); }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_ii_get(handle: HashMapIIHandle, key: i32) -> OptionI32 {
    if handle.is_null() { return OptionI32 { value: 0, is_some: false }; }
    match (*handle).get(&key) {
        Some(&v) => OptionI32 { value: v, is_some: true },
        None => OptionI32 { value: 0, is_some: false },
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_ii_len(handle: HashMapIIHandle) -> usize {
    if handle.is_null() { return 0; }
    (*handle).len()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn hashmap_ii_drop(handle: HashMapIIHandle) {
    if !handle.is_null() { drop(Box::from_raw(handle)); }
}

// =============================================================================
// Slice utilities
// =============================================================================

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn slice_i32_sum(data: *const i32, len: usize) -> i64 {
    if data.is_null() { return 0; }
    let slice = std::slice::from_raw_parts(data, len);
    slice.iter().map(|&x| x as i64).sum()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn slice_i32_sort(data: *mut i32, len: usize) {
    if data.is_null() { return; }
    let slice = std::slice::from_raw_parts_mut(data, len);
    slice.sort();
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn slice_f64_sum(data: *const f64, len: usize) -> f64 {
    if data.is_null() { return 0.0; }
    let slice = std::slice::from_raw_parts(data, len);
    slice.iter().sum()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn slice_f64_mean(data: *const f64, len: usize) -> f64 {
    if data.is_null() || len == 0 { return 0.0; }
    let slice = std::slice::from_raw_parts(data, len);
    let sum: f64 = slice.iter().sum();
    sum / len as f64
}
