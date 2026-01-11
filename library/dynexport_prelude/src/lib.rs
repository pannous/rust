//! Pre-instantiated generic exports for dynamic linking.
//!
//! This crate provides stable, dynamically-linkable exports for common generic
//! instantiations like `Vec<u8>`, `Option<i32>`, `HashMap<String, String>`, etc.
//!
//! # Naming Convention
//!
//! Symbols follow the pattern: `{type}_{element}_{method}`
//!
//! Examples:
//! - `vec_u8_new` - Creates a new `Vec<u8>`
//! - `vec_string_push` - Pushes to `Vec<String>`
//! - `option_i32_unwrap` - Unwraps `Option<i32>`
//!
//! # Memory Management
//!
//! Opaque handle types are used to pass ownership across the FFI boundary:
//! - `VecHandle<T>` - Owned `Vec<T>`
//! - `OptionHandle<T>` - Owned `Option<T>`
//! - `StringHandle` - Owned `String`
//!
//! Callers are responsible for calling the appropriate `_drop` function.
//!
//! # Usage
//!
//! ```ignore
//! // From C or dynamic loader:
//! VecU8Handle vec = vec_u8_new();
//! vec_u8_push(vec, 42);
//! size_t len = vec_u8_len(vec);
//! vec_u8_drop(vec);
//! ```

#![allow(clippy::missing_safety_doc)]

mod vec_exports;
mod option_exports;
mod result_exports;
mod string_exports;
mod hashmap_exports;
mod slice_exports;

pub use vec_exports::*;
pub use option_exports::*;
pub use result_exports::*;
pub use string_exports::*;
pub use hashmap_exports::*;
pub use slice_exports::*;

/// Version of the prelude ABI.
#[dynexport]
#[no_mangle]
pub static DYNEXPORT_PRELUDE_VERSION: u32 = 1;

/// Compiler version hash for ABI compatibility checking.
#[dynexport]
#[no_mangle]
pub static DYNEXPORT_COMPILER_HASH: u32 = {
    // Use a fixed version string when not built with Cargo
    const VERSION: &str = option_env!("CARGO_PKG_VERSION").unwrap_or("0.1.0");
    const fn simple_hash(s: &str) -> u32 {
        let bytes = s.as_bytes();
        let mut hash: u32 = 5381;
        let mut i = 0;
        while i < bytes.len() {
            hash = hash.wrapping_mul(33).wrapping_add(bytes[i] as u32);
            i += 1;
        }
        hash
    }
    simple_hash(VERSION)
};
