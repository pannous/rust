//! Vec<T> exports for common element types.

use std::ptr;

// =============================================================================
// Vec<u8> - Common for binary data
// =============================================================================

/// Opaque handle to Vec<u8>
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
    if !handle.is_null() {
        (*handle).push(value);
    }
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
pub unsafe extern "C" fn vec_u8_capacity(handle: VecU8Handle) -> usize {
    if handle.is_null() { return 0; }
    (*handle).capacity()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_u8_is_empty(handle: VecU8Handle) -> bool {
    if handle.is_null() { return true; }
    (*handle).is_empty()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_u8_clear(handle: VecU8Handle) {
    if !handle.is_null() {
        (*handle).clear();
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_u8_get(handle: VecU8Handle, index: usize) -> u8 {
    if handle.is_null() { return 0; }
    (*handle).get(index).copied().unwrap_or(0)
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_u8_set(handle: VecU8Handle, index: usize, value: u8) -> bool {
    if handle.is_null() { return false; }
    if let Some(elem) = (*handle).get_mut(index) {
        *elem = value;
        true
    } else {
        false
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_u8_as_ptr(handle: VecU8Handle) -> *const u8 {
    if handle.is_null() { return ptr::null(); }
    (*handle).as_ptr()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_u8_from_slice(data: *const u8, len: usize) -> VecU8Handle {
    if data.is_null() { return Box::into_raw(Box::new(Vec::new())); }
    let slice = std::slice::from_raw_parts(data, len);
    Box::into_raw(Box::new(slice.to_vec()))
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_u8_drop(handle: VecU8Handle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

// =============================================================================
// Vec<i32> - Common for integer arrays
// =============================================================================

pub type VecI32Handle = *mut Vec<i32>;

#[dynexport]
#[no_mangle]
pub extern "C" fn vec_i32_new() -> VecI32Handle {
    Box::into_raw(Box::new(Vec::<i32>::new()))
}

#[dynexport]
#[no_mangle]
pub extern "C" fn vec_i32_with_capacity(capacity: usize) -> VecI32Handle {
    Box::into_raw(Box::new(Vec::<i32>::with_capacity(capacity)))
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_i32_push(handle: VecI32Handle, value: i32) {
    if !handle.is_null() {
        (*handle).push(value);
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_i32_pop(handle: VecI32Handle) -> i32 {
    if handle.is_null() { return 0; }
    (*handle).pop().unwrap_or(0)
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
pub unsafe extern "C" fn vec_i32_set(handle: VecI32Handle, index: usize, value: i32) -> bool {
    if handle.is_null() { return false; }
    if let Some(elem) = (*handle).get_mut(index) {
        *elem = value;
        true
    } else {
        false
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_i32_as_ptr(handle: VecI32Handle) -> *const i32 {
    if handle.is_null() { return ptr::null(); }
    (*handle).as_ptr()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_i32_drop(handle: VecI32Handle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

// =============================================================================
// Vec<i64> - For 64-bit integers
// =============================================================================

pub type VecI64Handle = *mut Vec<i64>;

#[dynexport]
#[no_mangle]
pub extern "C" fn vec_i64_new() -> VecI64Handle {
    Box::into_raw(Box::new(Vec::<i64>::new()))
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_i64_push(handle: VecI64Handle, value: i64) {
    if !handle.is_null() {
        (*handle).push(value);
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_i64_len(handle: VecI64Handle) -> usize {
    if handle.is_null() { return 0; }
    (*handle).len()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_i64_get(handle: VecI64Handle, index: usize) -> i64 {
    if handle.is_null() { return 0; }
    (*handle).get(index).copied().unwrap_or(0)
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_i64_drop(handle: VecI64Handle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

// =============================================================================
// Vec<f64> - For floating point arrays
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
    if !handle.is_null() {
        (*handle).push(value);
    }
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
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

// =============================================================================
// Vec<usize> - For size/index arrays
// =============================================================================

pub type VecUsizeHandle = *mut Vec<usize>;

#[dynexport]
#[no_mangle]
pub extern "C" fn vec_usize_new() -> VecUsizeHandle {
    Box::into_raw(Box::new(Vec::<usize>::new()))
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_usize_push(handle: VecUsizeHandle, value: usize) {
    if !handle.is_null() {
        (*handle).push(value);
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_usize_len(handle: VecUsizeHandle) -> usize {
    if handle.is_null() { return 0; }
    (*handle).len()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_usize_get(handle: VecUsizeHandle, index: usize) -> usize {
    if handle.is_null() { return 0; }
    (*handle).get(index).copied().unwrap_or(0)
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_usize_drop(handle: VecUsizeHandle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}
