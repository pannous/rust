//! Slice utility exports.

use std::ptr;

// =============================================================================
// Slice utilities for [u8]
// =============================================================================

/// Copy bytes from a slice. Returns bytes written.
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn slice_u8_copy(
    src: *const u8,
    src_len: usize,
    dst: *mut u8,
    dst_len: usize,
) -> usize {
    if src.is_null() || dst.is_null() { return 0; }
    let copy_len = src_len.min(dst_len);
    ptr::copy_nonoverlapping(src, dst, copy_len);
    copy_len
}

/// Compare two byte slices. Returns 0 if equal, -1 if a < b, 1 if a > b.
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn slice_u8_cmp(
    a: *const u8,
    a_len: usize,
    b: *const u8,
    b_len: usize,
) -> i32 {
    if a.is_null() && b.is_null() { return 0; }
    if a.is_null() { return -1; }
    if b.is_null() { return 1; }

    let a = std::slice::from_raw_parts(a, a_len);
    let b = std::slice::from_raw_parts(b, b_len);

    match a.cmp(b) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

/// Check if two byte slices are equal.
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn slice_u8_eq(
    a: *const u8,
    a_len: usize,
    b: *const u8,
    b_len: usize,
) -> bool {
    if a_len != b_len { return false; }
    if a.is_null() && b.is_null() { return true; }
    if a.is_null() || b.is_null() { return false; }

    let a = std::slice::from_raw_parts(a, a_len);
    let b = std::slice::from_raw_parts(b, b_len);
    a == b
}

/// Find byte in slice. Returns index or usize::MAX if not found.
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn slice_u8_find(data: *const u8, len: usize, needle: u8) -> usize {
    if data.is_null() { return usize::MAX; }
    let slice = std::slice::from_raw_parts(data, len);
    slice.iter().position(|&x| x == needle).unwrap_or(usize::MAX)
}

/// Fill slice with value.
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn slice_u8_fill(data: *mut u8, len: usize, value: u8) {
    if data.is_null() { return; }
    let slice = std::slice::from_raw_parts_mut(data, len);
    slice.fill(value);
}

/// Reverse slice in place.
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn slice_u8_reverse(data: *mut u8, len: usize) {
    if data.is_null() { return; }
    let slice = std::slice::from_raw_parts_mut(data, len);
    slice.reverse();
}

// =============================================================================
// Slice utilities for [i32]
// =============================================================================

/// Sum of i32 slice.
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn slice_i32_sum(data: *const i32, len: usize) -> i64 {
    if data.is_null() { return 0; }
    let slice = std::slice::from_raw_parts(data, len);
    slice.iter().map(|&x| x as i64).sum()
}

/// Find max in i32 slice. Returns i32::MIN if empty.
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn slice_i32_max(data: *const i32, len: usize) -> i32 {
    if data.is_null() || len == 0 { return i32::MIN; }
    let slice = std::slice::from_raw_parts(data, len);
    *slice.iter().max().unwrap_or(&i32::MIN)
}

/// Find min in i32 slice. Returns i32::MAX if empty.
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn slice_i32_min(data: *const i32, len: usize) -> i32 {
    if data.is_null() || len == 0 { return i32::MAX; }
    let slice = std::slice::from_raw_parts(data, len);
    *slice.iter().min().unwrap_or(&i32::MAX)
}

/// Sort i32 slice in place (ascending).
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn slice_i32_sort(data: *mut i32, len: usize) {
    if data.is_null() { return; }
    let slice = std::slice::from_raw_parts_mut(data, len);
    slice.sort();
}

/// Sort i32 slice in place (descending).
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn slice_i32_sort_desc(data: *mut i32, len: usize) {
    if data.is_null() { return; }
    let slice = std::slice::from_raw_parts_mut(data, len);
    slice.sort_by(|a, b| b.cmp(a));
}

/// Binary search in sorted i32 slice. Returns index or usize::MAX if not found.
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn slice_i32_binary_search(data: *const i32, len: usize, target: i32) -> usize {
    if data.is_null() { return usize::MAX; }
    let slice = std::slice::from_raw_parts(data, len);
    slice.binary_search(&target).unwrap_or(usize::MAX)
}

/// Reverse i32 slice in place.
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn slice_i32_reverse(data: *mut i32, len: usize) {
    if data.is_null() { return; }
    let slice = std::slice::from_raw_parts_mut(data, len);
    slice.reverse();
}

// =============================================================================
// Slice utilities for [f64]
// =============================================================================

/// Sum of f64 slice.
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn slice_f64_sum(data: *const f64, len: usize) -> f64 {
    if data.is_null() { return 0.0; }
    let slice = std::slice::from_raw_parts(data, len);
    slice.iter().sum()
}

/// Find max in f64 slice. Returns f64::NEG_INFINITY if empty.
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn slice_f64_max(data: *const f64, len: usize) -> f64 {
    if data.is_null() || len == 0 { return f64::NEG_INFINITY; }
    let slice = std::slice::from_raw_parts(data, len);
    slice.iter().copied().fold(f64::NEG_INFINITY, f64::max)
}

/// Find min in f64 slice. Returns f64::INFINITY if empty.
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn slice_f64_min(data: *const f64, len: usize) -> f64 {
    if data.is_null() || len == 0 { return f64::INFINITY; }
    let slice = std::slice::from_raw_parts(data, len);
    slice.iter().copied().fold(f64::INFINITY, f64::min)
}

/// Sort f64 slice in place.
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn slice_f64_sort(data: *mut f64, len: usize) {
    if data.is_null() { return; }
    let slice = std::slice::from_raw_parts_mut(data, len);
    slice.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
}

/// Calculate mean of f64 slice.
#[dynexport]
#[no_mangle]
pub unsafe extern "C" fn slice_f64_mean(data: *const f64, len: usize) -> f64 {
    if data.is_null() || len == 0 { return 0.0; }
    let slice = std::slice::from_raw_parts(data, len);
    let sum: f64 = slice.iter().sum();
    sum / len as f64
}
