//! Result<T, E> exports for common types.

use std::os::raw::c_char;
use std::ffi::{CStr, CString};
use std::ptr;
use super::string_exports::StringHandle;

// =============================================================================
// Result<i32, String> - Common for integer operations that can fail
// =============================================================================

/// FFI-safe Result<i32, String>
#[repr(C)]
pub struct ResultI32 {
    pub value: i32,
    pub is_ok: bool,
    pub error: *mut c_char, // Null if is_ok, otherwise owned error string
}

#[dynexport]
#[no_mangle]
pub extern "C" fn result_i32_ok(value: i32) -> ResultI32 {
    ResultI32 {
        value,
        is_ok: true,
        error: ptr::null_mut(),
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn result_i32_err(error: *const c_char) -> ResultI32 {
    let error_owned = if error.is_null() {
        CString::new("unknown error").unwrap().into_raw()
    } else {
        let s = CStr::from_ptr(error).to_string_lossy().into_owned();
        CString::new(s).unwrap().into_raw()
    };
    ResultI32 {
        value: 0,
        is_ok: false,
        error: error_owned,
    }
}

#[dynexport]
#[no_mangle]
pub extern "C" fn result_i32_is_ok(result: &ResultI32) -> bool {
    result.is_ok
}

#[dynexport]
#[no_mangle]
pub extern "C" fn result_i32_is_err(result: &ResultI32) -> bool {
    !result.is_ok
}

#[dynexport]
#[no_mangle]
pub extern "C" fn result_i32_unwrap(result: ResultI32) -> i32 {
    if result.is_ok {
        result.value
    } else {
        panic!("called unwrap on Err")
    }
}

#[dynexport]
#[no_mangle]
pub extern "C" fn result_i32_unwrap_or(result: ResultI32, default: i32) -> i32 {
    if result.is_ok {
        result.value
    } else {
        // Free the error string
        if !result.error.is_null() {
            unsafe { drop(CString::from_raw(result.error)); }
        }
        default
    }
}

/// Get error message. Returns null if Ok. Caller must not free.
#[dynexport]
#[no_mangle]
pub extern "C" fn result_i32_error(result: &ResultI32) -> *const c_char {
    if result.is_ok {
        ptr::null()
    } else {
        result.error as *const c_char
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn result_i32_drop(result: ResultI32) {
    if !result.error.is_null() {
        drop(CString::from_raw(result.error));
    }
}

// =============================================================================
// Result<i64, String>
// =============================================================================

#[repr(C)]
pub struct ResultI64 {
    pub value: i64,
    pub is_ok: bool,
    pub error: *mut c_char,
}

#[dynexport]
#[no_mangle]
pub extern "C" fn result_i64_ok(value: i64) -> ResultI64 {
    ResultI64 {
        value,
        is_ok: true,
        error: ptr::null_mut(),
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn result_i64_err(error: *const c_char) -> ResultI64 {
    let error_owned = if error.is_null() {
        CString::new("unknown error").unwrap().into_raw()
    } else {
        let s = CStr::from_ptr(error).to_string_lossy().into_owned();
        CString::new(s).unwrap().into_raw()
    };
    ResultI64 {
        value: 0,
        is_ok: false,
        error: error_owned,
    }
}

#[dynexport]
#[no_mangle]
pub extern "C" fn result_i64_unwrap_or(result: ResultI64, default: i64) -> i64 {
    if result.is_ok {
        result.value
    } else {
        if !result.error.is_null() {
            unsafe { drop(CString::from_raw(result.error)); }
        }
        default
    }
}

// =============================================================================
// Result<String, String>
// =============================================================================

/// Opaque handle to Result<String, String>
pub type ResultStringHandle = *mut Result<String, String>;

#[dynexport]
#[no_mangle]
pub extern "C" fn result_string_ok(s: StringHandle) -> ResultStringHandle {
    if s.is_null() {
        return Box::into_raw(Box::new(Ok(String::new())));
    }
    unsafe {
        let string = *Box::from_raw(s);
        Box::into_raw(Box::new(Ok(string)))
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn result_string_err(error: *const c_char) -> ResultStringHandle {
    let error_str = if error.is_null() {
        "unknown error".to_string()
    } else {
        CStr::from_ptr(error).to_string_lossy().into_owned()
    };
    Box::into_raw(Box::new(Err(error_str)))
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn result_string_is_ok(handle: ResultStringHandle) -> bool {
    if handle.is_null() { return false; }
    (*handle).is_ok()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn result_string_is_err(handle: ResultStringHandle) -> bool {
    if handle.is_null() { return true; }
    (*handle).is_err()
}

/// Unwrap Ok value. Consumes the Result.
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn result_string_unwrap(handle: ResultStringHandle) -> StringHandle {
    if handle.is_null() {
        panic!("null handle");
    }
    let result = *Box::from_raw(handle);
    match result {
        Ok(s) => Box::into_raw(Box::new(s)),
        Err(e) => panic!("called unwrap on Err: {}", e),
    }
}

/// Get error string. Returns null if Ok. Does not consume.
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn result_string_error_ptr(handle: ResultStringHandle) -> *const u8 {
    if handle.is_null() { return ptr::null(); }
    match &*handle {
        Ok(_) => ptr::null(),
        Err(e) => e.as_ptr(),
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn result_string_drop(handle: ResultStringHandle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

// =============================================================================
// Result<(), String> - For operations that can fail but return nothing
// =============================================================================

#[repr(C)]
pub struct ResultUnit {
    pub is_ok: bool,
    pub error: *mut c_char,
}

#[dynexport]
#[no_mangle]
pub extern "C" fn result_unit_ok() -> ResultUnit {
    ResultUnit {
        is_ok: true,
        error: ptr::null_mut(),
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn result_unit_err(error: *const c_char) -> ResultUnit {
    let error_owned = if error.is_null() {
        CString::new("unknown error").unwrap().into_raw()
    } else {
        let s = CStr::from_ptr(error).to_string_lossy().into_owned();
        CString::new(s).unwrap().into_raw()
    };
    ResultUnit {
        is_ok: false,
        error: error_owned,
    }
}

#[dynexport]
#[no_mangle]
pub extern "C" fn result_unit_is_ok(result: &ResultUnit) -> bool {
    result.is_ok
}

#[dynexport]
#[no_mangle]
pub extern "C" fn result_unit_error(result: &ResultUnit) -> *const c_char {
    if result.is_ok { ptr::null() } else { result.error as *const c_char }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn result_unit_drop(result: ResultUnit) {
    if !result.error.is_null() {
        drop(CString::from_raw(result.error));
    }
}
