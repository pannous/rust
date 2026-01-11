//! Option<T> exports for common types.

use std::ptr;
use super::string_exports::StringHandle;

// =============================================================================
// Option<i32>
// =============================================================================

/// Representation of Option<i32> for FFI.
/// is_some indicates whether the value is valid.
#[repr(C)]
pub struct OptionI32 {
    pub value: i32,
    pub is_some: bool,
}

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
pub extern "C" fn option_i32_is_some(opt: OptionI32) -> bool {
    opt.is_some
}

#[dynexport]
#[no_mangle]
pub extern "C" fn option_i32_is_none(opt: OptionI32) -> bool {
    !opt.is_some
}

#[dynexport]
#[no_mangle]
pub extern "C" fn option_i32_unwrap(opt: OptionI32) -> i32 {
    if opt.is_some {
        opt.value
    } else {
        panic!("called unwrap on None")
    }
}

#[dynexport]
#[no_mangle]
pub extern "C" fn option_i32_unwrap_or(opt: OptionI32, default: i32) -> i32 {
    if opt.is_some { opt.value } else { default }
}

// =============================================================================
// Option<i64>
// =============================================================================

#[repr(C)]
pub struct OptionI64 {
    pub value: i64,
    pub is_some: bool,
}

#[dynexport]
#[no_mangle]
pub extern "C" fn option_i64_some(value: i64) -> OptionI64 {
    OptionI64 { value, is_some: true }
}

#[dynexport]
#[no_mangle]
pub extern "C" fn option_i64_none() -> OptionI64 {
    OptionI64 { value: 0, is_some: false }
}

#[dynexport]
#[no_mangle]
pub extern "C" fn option_i64_unwrap_or(opt: OptionI64, default: i64) -> i64 {
    if opt.is_some { opt.value } else { default }
}

// =============================================================================
// Option<f64>
// =============================================================================

#[repr(C)]
pub struct OptionF64 {
    pub value: f64,
    pub is_some: bool,
}

#[dynexport]
#[no_mangle]
pub extern "C" fn option_f64_some(value: f64) -> OptionF64 {
    OptionF64 { value, is_some: true }
}

#[dynexport]
#[no_mangle]
pub extern "C" fn option_f64_none() -> OptionF64 {
    OptionF64 { value: 0.0, is_some: false }
}

#[dynexport]
#[no_mangle]
pub extern "C" fn option_f64_unwrap_or(opt: OptionF64, default: f64) -> f64 {
    if opt.is_some { opt.value } else { default }
}

// =============================================================================
// Option<usize>
// =============================================================================

#[repr(C)]
pub struct OptionUsize {
    pub value: usize,
    pub is_some: bool,
}

#[dynexport]
#[no_mangle]
pub extern "C" fn option_usize_some(value: usize) -> OptionUsize {
    OptionUsize { value, is_some: true }
}

#[dynexport]
#[no_mangle]
pub extern "C" fn option_usize_none() -> OptionUsize {
    OptionUsize { value: 0, is_some: false }
}

#[dynexport]
#[no_mangle]
pub extern "C" fn option_usize_unwrap_or(opt: OptionUsize, default: usize) -> usize {
    if opt.is_some { opt.value } else { default }
}

// =============================================================================
// Option<String> - uses handle for owned string
// =============================================================================

/// Opaque handle to Option<String>
pub type OptionStringHandle = *mut Option<String>;

#[dynexport]
#[no_mangle]
pub extern "C" fn option_string_some(s: StringHandle) -> OptionStringHandle {
    if s.is_null() {
        return Box::into_raw(Box::new(None));
    }
    unsafe {
        let string = *Box::from_raw(s);
        Box::into_raw(Box::new(Some(string)))
    }
}

#[dynexport]
#[no_mangle]
pub extern "C" fn option_string_none() -> OptionStringHandle {
    Box::into_raw(Box::new(None))
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn option_string_is_some(handle: OptionStringHandle) -> bool {
    if handle.is_null() { return false; }
    (*handle).is_some()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn option_string_is_none(handle: OptionStringHandle) -> bool {
    if handle.is_null() { return true; }
    (*handle).is_none()
}

/// Unwrap and return the String. Consumes the Option.
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn option_string_unwrap(handle: OptionStringHandle) -> StringHandle {
    if handle.is_null() {
        panic!("called unwrap on null handle");
    }
    let opt = *Box::from_raw(handle);
    match opt {
        Some(s) => Box::into_raw(Box::new(s)),
        None => panic!("called unwrap on None"),
    }
}

/// Get a reference to the string without consuming. Returns null if None.
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn option_string_as_ref(handle: OptionStringHandle) -> *const u8 {
    if handle.is_null() { return ptr::null(); }
    match &*handle {
        Some(s) => s.as_ptr(),
        None => ptr::null(),
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn option_string_drop(handle: OptionStringHandle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

// =============================================================================
// Option<bool>
// =============================================================================

#[repr(C)]
pub struct OptionBool {
    pub value: bool,
    pub is_some: bool,
}

#[dynexport]
#[no_mangle]
pub extern "C" fn option_bool_some(value: bool) -> OptionBool {
    OptionBool { value, is_some: true }
}

#[dynexport]
#[no_mangle]
pub extern "C" fn option_bool_none() -> OptionBool {
    OptionBool { value: false, is_some: false }
}

#[dynexport]
#[no_mangle]
pub extern "C" fn option_bool_unwrap_or(opt: OptionBool, default: bool) -> bool {
    if opt.is_some { opt.value } else { default }
}
