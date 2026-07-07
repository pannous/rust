// Val enum for dynamic typing in script mode.
//
// Enables heterogeneous collections and dynamic value handling.
// Note: Truthy trait must be defined before this file (loaded from truthy.rs).

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

// Untyped `{key: value, ...}` map literals desugar to `Map<V, N>`, backed by a
// fixed-size array (the key count is known at the literal's parse site).
// Unlike `std::collections::HashMap`, iterating a `Map` directly (`for key in map`)
// yields keys only; use `.pairs()` for `(key, value)` tuples. Being array-backed
// lets `Map` derive `Copy` when `V: Copy`, so `for key in map { ... map[key] ... }`
// works without explicitly borrowing `map`.
#[derive(Debug, Clone, Copy)]
pub struct Map<V, const N: usize>(pub [(&'static str, V); N]);

impl<V, const N: usize> Map<V, N> {
	pub fn from(pairs: [(&'static str, V); N]) -> Self { Map(pairs) }
	pub fn len(&self) -> usize { N }
	pub fn is_empty(&self) -> bool { N == 0 }
	fn keys(&self) -> [&'static str; N] { std::array::from_fn(|i| self.0[i].0) }
}

impl<V: Clone, const N: usize> Map<V, N> {
	pub fn pairs(&self) -> Vec<(&'static str, V)> { self.0.to_vec() }
}

impl<V, const N: usize> std::ops::Index<&str> for Map<V, N> {
	type Output = V;
	fn index(&self, key: &str) -> &V {
		self.0.iter().find(|(k, _)| *k == key).map(|(_, v)| v).expect("key not found in map")
	}
}

impl<V, const N: usize> IntoIterator for Map<V, N> {
	type Item = &'static str;
	type IntoIter = std::array::IntoIter<&'static str, N>;
	fn into_iter(self) -> Self::IntoIter {
		// Method-call syntax (`arr.into_iter()`) resolves to the by-reference
		// slice iterator on pre-2021 editions; UFCS syntax picks the by-value
		// array impl unambiguously regardless of edition.
		IntoIterator::into_iter(self.keys())
	}
}

impl<'a, V, const N: usize> IntoIterator for &'a Map<V, N> {
	type Item = &'static str;
	type IntoIter = std::array::IntoIter<&'static str, N>;
	fn into_iter(self) -> Self::IntoIter {
		IntoIterator::into_iter(self.keys())
	}
}

