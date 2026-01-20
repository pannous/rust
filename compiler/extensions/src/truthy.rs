//! Truthy trait for script mode.
//!
//! Allows values to be used in boolean contexts like Python/JavaScript.

#[allow(dead_code)]
pub trait Truthy {
	fn is_truthy(&self) -> bool;
}

impl Truthy for bool {
	fn is_truthy(&self) -> bool { *self }
}

// Integer types
impl Truthy for i8 { fn is_truthy(&self) -> bool { *self != 0 } }
impl Truthy for i16 { fn is_truthy(&self) -> bool { *self != 0 } }
impl Truthy for i32 { fn is_truthy(&self) -> bool { *self != 0 } }
impl Truthy for i64 { fn is_truthy(&self) -> bool { *self != 0 } }
impl Truthy for i128 { fn is_truthy(&self) -> bool { *self != 0 } }
impl Truthy for isize { fn is_truthy(&self) -> bool { *self != 0 } }
impl Truthy for u8 { fn is_truthy(&self) -> bool { *self != 0 } }
impl Truthy for u16 { fn is_truthy(&self) -> bool { *self != 0 } }
impl Truthy for u32 { fn is_truthy(&self) -> bool { *self != 0 } }
impl Truthy for u64 { fn is_truthy(&self) -> bool { *self != 0 } }
impl Truthy for u128 { fn is_truthy(&self) -> bool { *self != 0 } }
impl Truthy for usize { fn is_truthy(&self) -> bool { *self != 0 } }

// Float types
impl Truthy for f32 { fn is_truthy(&self) -> bool { *self != 0.0 } }
impl Truthy for f64 { fn is_truthy(&self) -> bool { *self != 0.0 } }

// String types
impl Truthy for &str { fn is_truthy(&self) -> bool { !self.is_empty() } }
impl Truthy for String { fn is_truthy(&self) -> bool { !self.is_empty() } }

// Collections
impl<T> Truthy for Vec<T> { fn is_truthy(&self) -> bool { !self.is_empty() } }
impl<T> Truthy for Option<T> { fn is_truthy(&self) -> bool { self.is_some() } }
