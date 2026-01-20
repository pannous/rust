//! List/slice extension methods for script mode.
//!
//! Provides convenient collection methods with intuitive synonyms.

#[allow(dead_code)]
pub trait ScriptSliceExt<T: Clone> { // todo rename to SliceExtensions
	// Map synonyms - transform each element
	fn mapped<U, F: Fn(T) -> U>(&self, f: F) -> Vec<U>;
	fn apply<U, F: Fn(T) -> U>(&self, f: F) -> Vec<U>;
	fn transform<U, F: Fn(T) -> U>(&self, f: F) -> Vec<U>;
	fn convert<U, F: Fn(T) -> U>(&self, f: F) -> Vec<U>;

	// Filter synonyms - select elements matching predicate
	fn filtered<F: Fn(&T) -> bool>(&self, f: F) -> Vec<T>;
	fn select<F: Fn(&T) -> bool>(&self, f: F) -> Vec<T>;
	fn chose<F: Fn(&T) -> bool>(&self, f: F) -> Vec<T>;
	fn that<F: Fn(&T) -> bool>(&self, f: F) -> Vec<T>;
	fn which<F: Fn(&T) -> bool>(&self, f: F) -> Vec<T>;

	// Element access
	fn first_cloned(&self) -> Option<T>;
	fn shift(&self) -> Option<T>;

	// Enumeration
	fn pairs(&self) -> Vec<(usize, T)>;
}

impl<T: Clone, S: AsRef<[T]>> ScriptSliceExt<T> for S {
	fn mapped<U, F: Fn(T) -> U>(&self, f: F) -> Vec<U> {
		self.as_ref().iter().cloned().map(f).collect()
	}
	fn apply<U, F: Fn(T) -> U>(&self, f: F) -> Vec<U> { self.mapped(f) }
	fn transform<U, F: Fn(T) -> U>(&self, f: F) -> Vec<U> { self.mapped(f) }
	fn convert<U, F: Fn(T) -> U>(&self, f: F) -> Vec<U> { self.mapped(f) }

	fn filtered<F: Fn(&T) -> bool>(&self, f: F) -> Vec<T> {
		self.as_ref().iter().filter(|x| f(x)).cloned().collect()
	}
	fn select<F: Fn(&T) -> bool>(&self, f: F) -> Vec<T> { self.filtered(f) }
	fn chose<F: Fn(&T) -> bool>(&self, f: F) -> Vec<T> { self.filtered(f) }
	fn that<F: Fn(&T) -> bool>(&self, f: F) -> Vec<T> { self.filtered(f) }
	fn which<F: Fn(&T) -> bool>(&self, f: F) -> Vec<T> { self.filtered(f) }

	fn first_cloned(&self) -> Option<T> {
		self.as_ref().first().cloned()
	}
	fn shift(&self) -> Option<T> { self.first_cloned() }

	fn pairs(&self) -> Vec<(usize, T)> {
		self.as_ref().iter().cloned().enumerate().collect()
	}
}

#[allow(dead_code)]
fn slice_eq<T: PartialEq, A: AsRef<[T]>, B: AsRef<[T]>>(a: &A, b: &B) -> bool {
	a.as_ref() == b.as_ref()
}
