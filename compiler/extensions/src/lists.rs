// List/slice/vec extension methods for script mode.
//
// Provides convenient collection methods with intuitive synonyms.


#[allow(dead_code)]
pub trait ListExtensions<T: Clone> {
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

	// Slicing and copying
	fn slice(&self, start: usize, end: usize) -> Vec<T>;
	fn copy(&self) -> Vec<T>;

	// Adding elements (non-mutating, returns new vec)
	fn append(&self, item: T) -> Vec<T>;
	fn prepend(&self, item: T) -> Vec<T>;
	fn insert(&self, index: usize, item: T) -> Vec<T>;

	// Non-mutating reverse
	fn reversed(&self) -> Vec<T>;

	// Index finding
	fn indexOf(&self, item: &T) -> i64 where T: PartialEq;
	fn index_of(&self, item: &T) -> i64 where T: PartialEq;

	// Sorting (returns new vec)
	fn sorted(&self) -> Vec<T> where T: Ord;
	fn sortDesc(&self) -> Vec<T> where T: Ord;
	fn sort_desc(&self) -> Vec<T> where T: Ord;
}

impl<T: Clone, S: AsRef<[T]>> ListExtensions<T> for S {
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

	// Slicing and copying
	fn slice(&self, start: usize, end: usize) -> Vec<T> {
		self.as_ref()[start..end].to_vec()
	}
	fn copy(&self) -> Vec<T> { self.as_ref().to_vec() }

	// Adding elements (non-mutating)
	fn append(&self, item: T) -> Vec<T> {
		let mut v = self.as_ref().to_vec();
		v.push(item);
		v
	}
	fn prepend(&self, item: T) -> Vec<T> {
		let mut v = vec![item];
		v.extend(self.as_ref().iter().cloned());
		v
	}
	fn insert(&self, index: usize, item: T) -> Vec<T> {
		let mut v = self.as_ref().to_vec();
		Vec::insert(&mut v, index, item);
		v
	}

	// Non-mutating reverse
	fn reversed(&self) -> Vec<T> {
		self.as_ref().iter().rev().cloned().collect()
	}

	// Index finding - returns -1 if not found (like JS/Python convention)
	fn indexOf(&self, item: &T) -> i64 where T: PartialEq {
		self.as_ref().iter().position(|x| x == item).map(|i| i as i64).unwrap_or(-1)
	}
	fn index_of(&self, item: &T) -> i64 where T: PartialEq { self.indexOf(item) }

	// Sorting (returns new sorted vec)
	fn sorted(&self) -> Vec<T> where T: Ord {
		let mut v = self.as_ref().to_vec();
		v.sort();
		v
	}
	fn sortDesc(&self) -> Vec<T> where T: Ord {
		let mut v = self.as_ref().to_vec();
		v.sort();
		v.reverse();
		v
	}
	fn sort_desc(&self) -> Vec<T> where T: Ord { self.sortDesc() }
}

// Free function for len(collection)
#[allow(dead_code)]
pub fn len<T, S: AsRef<[T]>>(s: S) -> usize {
	s.as_ref().len()
}

#[allow(dead_code)]
pub fn slice_eq<T: PartialEq, A: AsRef<[T]>, B: AsRef<[T]>>(a: &A, b: &B) -> bool {
	a.as_ref() == b.as_ref()
}

// Separate trait for size/length on slices/vecs to avoid conflict with StringExtensions
// (str also implements AsRef<[u8]> so blanket impl would overlap)
#[allow(dead_code)]
pub trait SliceSizeExt {
	fn size(&self) -> usize;
	fn length(&self) -> usize;
}

impl<T> SliceSizeExt for [T] {
	fn size(&self) -> usize { self.len() }
	fn length(&self) -> usize { self.len() }
}

impl<T> SliceSizeExt for Vec<T> {
	fn size(&self) -> usize { self.len() }
	fn length(&self) -> usize { self.len() }
}
