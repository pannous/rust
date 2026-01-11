//! String exports for dynamic linking.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

/// Opaque handle to String
pub type StringHandle = *mut String;

/// Opaque handle to Vec<String>
pub type VecStringHandle = *mut Vec<String>;

// =============================================================================
// String operations
// =============================================================================

#[dynexport]
#[no_mangle]
pub extern "C" fn string_new() -> StringHandle {
    Box::into_raw(Box::new(String::new()))
}

#[dynexport]
#[no_mangle]
pub extern "C" fn string_with_capacity(capacity: usize) -> StringHandle {
    Box::into_raw(Box::new(String::with_capacity(capacity)))
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn string_from_cstr(s: *const c_char) -> StringHandle {
    if s.is_null() {
        return Box::into_raw(Box::new(String::new()));
    }
    let cstr = CStr::from_ptr(s);
    match cstr.to_str() {
        Ok(s) => Box::into_raw(Box::new(s.to_string())),
        Err(_) => Box::into_raw(Box::new(String::new())),
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn string_from_utf8(data: *const u8, len: usize) -> StringHandle {
    if data.is_null() {
        return Box::into_raw(Box::new(String::new()));
    }
    let slice = std::slice::from_raw_parts(data, len);
    match String::from_utf8(slice.to_vec()) {
        Ok(s) => Box::into_raw(Box::new(s)),
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
pub unsafe extern "C" fn string_capacity(handle: StringHandle) -> usize {
    if handle.is_null() { return 0; }
    (*handle).capacity()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn string_is_empty(handle: StringHandle) -> bool {
    if handle.is_null() { return true; }
    (*handle).is_empty()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn string_push_str(handle: StringHandle, s: *const c_char) {
    if handle.is_null() || s.is_null() { return; }
    if let Ok(s) = CStr::from_ptr(s).to_str() {
        (*handle).push_str(s);
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn string_push_char(handle: StringHandle, c: u32) {
    if handle.is_null() { return; }
    if let Some(ch) = char::from_u32(c) {
        (*handle).push(ch);
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn string_clear(handle: StringHandle) {
    if !handle.is_null() {
        (*handle).clear();
    }
}

/// Returns a null-terminated C string. Caller must free with string_free_cstr.
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
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn string_as_ptr(handle: StringHandle) -> *const u8 {
    if handle.is_null() { return ptr::null(); }
    (*handle).as_ptr()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn string_clone(handle: StringHandle) -> StringHandle {
    if handle.is_null() { return Box::into_raw(Box::new(String::new())); }
    Box::into_raw(Box::new((*handle).clone()))
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn string_eq(a: StringHandle, b: StringHandle) -> bool {
    if a.is_null() || b.is_null() { return a.is_null() && b.is_null(); }
    *a == *b
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn string_contains(handle: StringHandle, needle: *const c_char) -> bool {
    if handle.is_null() || needle.is_null() { return false; }
    if let Ok(needle) = CStr::from_ptr(needle).to_str() {
        (*handle).contains(needle)
    } else {
        false
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn string_starts_with(handle: StringHandle, prefix: *const c_char) -> bool {
    if handle.is_null() || prefix.is_null() { return false; }
    if let Ok(prefix) = CStr::from_ptr(prefix).to_str() {
        (*handle).starts_with(prefix)
    } else {
        false
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn string_ends_with(handle: StringHandle, suffix: *const c_char) -> bool {
    if handle.is_null() || suffix.is_null() { return false; }
    if let Ok(suffix) = CStr::from_ptr(suffix).to_str() {
        (*handle).ends_with(suffix)
    } else {
        false
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn string_trim(handle: StringHandle) -> StringHandle {
    if handle.is_null() { return Box::into_raw(Box::new(String::new())); }
    Box::into_raw(Box::new((*handle).trim().to_string()))
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn string_to_lowercase(handle: StringHandle) -> StringHandle {
    if handle.is_null() { return Box::into_raw(Box::new(String::new())); }
    Box::into_raw(Box::new((*handle).to_lowercase()))
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn string_to_uppercase(handle: StringHandle) -> StringHandle {
    if handle.is_null() { return Box::into_raw(Box::new(String::new())); }
    Box::into_raw(Box::new((*handle).to_uppercase()))
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn string_replace(
    handle: StringHandle,
    from: *const c_char,
    to: *const c_char
) -> StringHandle {
    if handle.is_null() || from.is_null() || to.is_null() {
        return Box::into_raw(Box::new(String::new()));
    }
    let from = match CStr::from_ptr(from).to_str() {
        Ok(s) => s,
        Err(_) => return Box::into_raw(Box::new(String::new())),
    };
    let to = match CStr::from_ptr(to).to_str() {
        Ok(s) => s,
        Err(_) => return Box::into_raw(Box::new(String::new())),
    };
    Box::into_raw(Box::new((*handle).replace(from, to)))
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn string_drop(handle: StringHandle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

// =============================================================================
// Vec<String> operations
// =============================================================================

#[dynexport]
#[no_mangle]
pub extern "C" fn vec_string_new() -> VecStringHandle {
    Box::into_raw(Box::new(Vec::<String>::new()))
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_string_push(handle: VecStringHandle, s: StringHandle) {
    if handle.is_null() || s.is_null() { return; }
    // Take ownership of the string
    let string = *Box::from_raw(s);
    (*handle).push(string);
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_string_push_cstr(handle: VecStringHandle, s: *const c_char) {
    if handle.is_null() || s.is_null() { return; }
    if let Ok(s) = CStr::from_ptr(s).to_str() {
        (*handle).push(s.to_string());
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_string_len(handle: VecStringHandle) -> usize {
    if handle.is_null() { return 0; }
    (*handle).len()
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_string_get(handle: VecStringHandle, index: usize) -> StringHandle {
    if handle.is_null() { return ptr::null_mut(); }
    match (*handle).get(index) {
        Some(s) => Box::into_raw(Box::new(s.clone())),
        None => ptr::null_mut(),
    }
}

/// Returns a borrowed C string pointer. Valid until vec is modified.
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_string_get_cstr(handle: VecStringHandle, index: usize) -> *const c_char {
    if handle.is_null() { return ptr::null(); }
    match (*handle).get(index) {
        Some(s) => s.as_ptr() as *const c_char,
        None => ptr::null(),
    }
}

#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_string_drop(handle: VecStringHandle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

/// Split a string by delimiter, returns Vec<String>
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn string_split(handle: StringHandle, delimiter: *const c_char) -> VecStringHandle {
    if handle.is_null() || delimiter.is_null() {
        return Box::into_raw(Box::new(Vec::new()));
    }
    let delim = match CStr::from_ptr(delimiter).to_str() {
        Ok(s) => s,
        Err(_) => return Box::into_raw(Box::new(Vec::new())),
    };
    let parts: Vec<String> = (*handle).split(delim).map(|s| s.to_string()).collect();
    Box::into_raw(Box::new(parts))
}

/// Join Vec<String> with delimiter
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn vec_string_join(handle: VecStringHandle, delimiter: *const c_char) -> StringHandle {
    if handle.is_null() || delimiter.is_null() {
        return Box::into_raw(Box::new(String::new()));
    }
    let delim = match CStr::from_ptr(delimiter).to_str() {
        Ok(s) => s,
        Err(_) => return Box::into_raw(Box::new(String::new())),
    };
    Box::into_raw(Box::new((*handle).join(delim)))
}
