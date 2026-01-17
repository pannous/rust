//! Script mode AST transformers.
//!
//! This module contains various AST transformation utilities for script mode,
//! such as generating extension traits for convenient method syntax.

pub(crate) mod filter;

#[allow(unused_imports)]
pub(crate) use filter::build_slice_helpers;
