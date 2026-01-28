// List/slice/vec extension methods for script mode.
//
// Provides convenient collection methods with intuitive synonyms.

// Array accessor methods (head, tail, first, last, etc.)
// Implemented specifically for arrays/slices to avoid conflict with StringExtensions
#[allow(dead_code)]
pub trait ArrayExtensions<T: Clone> {
	// First element access - returns Option<T> (cloned)
	// Note: 'first' and 'last' methods removed to avoid shadowing std library methods
	fn head(&self) -> Option<T>;
	fn start(&self) -> Option<T>;
	fn begin(&self) -> Option<T>;

	// Last element access - returns Option<T> (cloned)
	fn tail(&self) -> Option<T>;
	fn end(&self) -> Option<T>;
}

impl<T: Clone> ArrayExtensions<T> for [T] {
	fn head(&self) -> Option<T> { <[T]>::first(self).cloned() }
	fn start(&self) -> Option<T> { <[T]>::first(self).cloned() }
	fn begin(&self) -> Option<T> { <[T]>::first(self).cloned() }

	fn tail(&self) -> Option<T> { <[T]>::last(self).cloned() }
	fn end(&self) -> Option<T> { <[T]>::last(self).cloned() }
}

impl<T: Clone> ArrayExtensions<T> for Vec<T> {
	fn head(&self) -> Option<T> { self.as_slice().head() }
	fn start(&self) -> Option<T> { self.as_slice().start() }
	fn begin(&self) -> Option<T> { self.as_slice().begin() }

	fn tail(&self) -> Option<T> { self.as_slice().tail() }
	fn end(&self) -> Option<T> { self.as_slice().end() }
}

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

	// Inverse filter utilities
	// `without` returns a new Vec with all elements equal to `item` removed
	fn without(&self, item: &T) -> Vec<T> where T: PartialEq;

	// `except` returns a new Vec with elements matching predicate removed
	fn except<F: Fn(&T) -> bool>(&self, f: F) -> Vec<T>;

	// Element access (non-mutating)
	fn first_cloned(&self) -> Option<T>;

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
	#[allow(nonstandard_style)]
	fn indexOf(&self, item: &T) -> i64 where T: PartialEq;
	fn index_of(&self, item: &T) -> i64 where T: PartialEq;

	// Sorting (returns new vec)
	fn sorted(&self) -> Vec<T> where T: Ord;
	#[allow(nonstandard_style)]
	fn sortDesc(&self) -> Vec<T> where T: Ord;
	fn sort_desc(&self) -> Vec<T> where T: Ord;

	// Random utilities
	// `random` returns a single random element (cloned) or `None` for empty slices
	fn random(&self) -> Option<T>;

	// `shuffle` returns a new Vec with the elements shuffled (non-mutating)
	fn shuffle(&self) -> Vec<T>;
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

	fn without(&self, item: &T) -> Vec<T> where T: PartialEq {
		self.as_ref().iter().filter(|x| x != &item).cloned().collect()
	}

	fn except<F: Fn(&T) -> bool>(&self, f: F) -> Vec<T> {
		self.as_ref().iter().filter(|x| !f(x)).cloned().collect()
	}

	fn first_cloned(&self) -> Option<T> {
		self.as_ref().first().cloned()
	}

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
	#[allow(nonstandard_style)]
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
	#[allow(nonstandard_style)]
	fn sortDesc(&self) -> Vec<T> where T: Ord {
		let mut v = self.as_ref().to_vec();
		v.sort();
		v.reverse();
		v
	}
	fn sort_desc(&self) -> Vec<T> where T: Ord { self.sortDesc() }

	// Random utilities implementation
	fn random(&self) -> Option<T> {
		let slice = self.as_ref();
		if slice.is_empty() { return None }
		let idx = rand_index(slice.len());
		slice.get(idx).cloned()
	}

	fn shuffle(&self) -> Vec<T> {
		let mut v = self.as_ref().to_vec();
		// Fisher-Yates shuffle
		let mut n = v.len();
		while n > 1 {
			let j = rand_index(n);
			n -= 1;
			v.swap(n, j);
		}
		v
	}
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

// Mutating operations for Vec only (not slices)
#[allow(dead_code)]
pub trait VecMutExtensions<T> {
	fn shift(&mut self) -> Option<T>;
	// Remove last element (alias of Vec::pop)
	fn pop_last(&mut self) -> Option<T>;
	// Remove first element (remove index 0)
	fn pop_first(&mut self) -> Option<T>;
	// Delete element at index, returning it if in-bounds
	fn delete_at(&mut self, index: usize) -> Option<T>;
	// Delete a range [start, end) of elements and return them as a Vec
	fn delete_range(&mut self, start: usize, end: usize) -> Vec<T>;
	// Replace element at index with `item`, returning the old value if index in-bounds
	fn replace_at(&mut self, index: usize, item: T) -> Option<T>;
	// Swap-remove at index (may change order), returning removed element if in-bounds
	fn swap_remove_at(&mut self, index: usize) -> Option<T>;
}

impl<T> VecMutExtensions<T> for Vec<T> {
	fn shift(&mut self) -> Option<T> {
		if self.is_empty() { None } else { Some(self.remove(0)) }
	}
	fn pop_last(&mut self) -> Option<T> { self.pop() }

	fn pop_first(&mut self) -> Option<T> { if self.is_empty() { None } else { Some(self.remove(0)) } }

	fn delete_at(&mut self, index: usize) -> Option<T> {
		if index < self.len() { Some(self.remove(index)) } else { None }
	}

	fn delete_range(&mut self, start: usize, end: usize) -> Vec<T> {
		let len = self.len();
		if start >= len || start >= end { return Vec::new(); }
		let end = end.min(len);
		// Drain and collect
		self.drain(start..end).collect()
	}

	fn replace_at(&mut self, index: usize, item: T) -> Option<T> {
		if index < self.len() {
			let old = std::mem::replace(&mut self[index], item);
			Some(old)
		} else { None }
	}

	fn swap_remove_at(&mut self, index: usize) -> Option<T> {
		if index < self.len() { Some(self.swap_remove(index)) } else { None }
	}
}
