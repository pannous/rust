//! Val enum for dynamic typing in script mode.
//!
//! Enables heterogeneous collections and dynamic value handling.

use super::truthy::Truthy;

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum Val {
	Str(String),
	Int(i64),
	Float(f64),
	Bool(bool),
	List(Vec<Val>),
	Nil,
}

impl std::fmt::Display for Val {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Val::Str(s) => write!(f, "{}", s),
			Val::Int(n) => write!(f, "{}", n),
			Val::Float(n) => write!(f, "{}", n),
			Val::Bool(b) => write!(f, "{}", b),
			Val::List(v) => write!(f, "{:?}", v),
			Val::Nil => write!(f, "nil"),
		}
	}
}

// From implementations for various types
impl From<&str> for Val {
	fn from(s: &str) -> Self { Val::Str(s.to_string()) }
}
impl From<String> for Val {
	fn from(s: String) -> Self { Val::Str(s) }
}
impl From<i64> for Val {
	fn from(n: i64) -> Self { Val::Int(n) }
}
impl From<i32> for Val {
	fn from(n: i32) -> Self { Val::Int(n as i64) }
}
impl From<f64> for Val {
	fn from(n: f64) -> Self { Val::Float(n) }
}
impl From<f32> for Val {
	fn from(n: f32) -> Self { Val::Float(n as f64) }
}
impl From<bool> for Val {
	fn from(b: bool) -> Self { Val::Bool(b) }
}
impl From<char> for Val {
	fn from(c: char) -> Self { Val::Str(c.to_string()) }
}

// PartialEq with char for comparison
impl PartialEq<char> for Val {
	fn eq(&self, other: &char) -> bool {
		match self {
			Val::Str(s) => s.len() == 1 && s.chars().next() == Some(*other),
			_ => false,
		}
	}
}

// Truthy implementation
impl Truthy for Val {
	fn is_truthy(&self) -> bool {
		match self {
			Val::Str(s) => !s.is_empty(),
			Val::Int(n) => *n != 0,
			Val::Float(n) => *n != 0.0,
			Val::Bool(b) => *b,
			Val::List(v) => !v.is_empty(),
			Val::Nil => false,
		}
	}
}
