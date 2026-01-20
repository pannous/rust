//! Script mode extensions library.
//!
//! This crate contains Rust source code that gets injected into scripts
//! by the script harness. The code is read as text, parsed into AST,
//! and injected into the script's module.
//!
//! **Note**: This crate is NOT compiled as a regular library. It exists
//! primarily for IDE support and to make the extension code readable.

pub mod strings;
pub mod lists;
pub mod truthy;
pub mod val;
pub mod numbers;
pub mod macros;
