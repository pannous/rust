//! String extension methods for script mode.
//!
//! Provides convenient string methods with intuitive synonyms.

#[allow(dead_code)]
pub trait ScriptStrExt { // todo rename to StringExtensions
	// Element access - first character
	fn first(&self) -> String;
	fn head(&self) -> String;
	fn start(&self) -> String;
	fn begin(&self) -> String;

	// Element access - last character
	fn last(&self) -> String;
	fn tail(&self) -> String;
	fn end(&self) -> String;

	// Transformation
	fn reverse(&self) -> String;

	// Length
	fn size(&self) -> usize;
	fn length(&self) -> usize;

	// Case conversion - uppercase
	fn upper(&self) -> String;
	fn to_upper(&self) -> String;
	fn toUpper(&self) -> String;
	fn uppercase(&self) -> String;

	// Case conversion - lowercase
	fn lower(&self) -> String;
	fn to_lower(&self) -> String;
	fn toLower(&self) -> String;
	fn lowercase(&self) -> String;

	// Search - contains synonyms
	fn includes(&self, pat: &str) -> bool;
	fn has(&self, pat: &str) -> bool;
	fn holds(&self, pat: &str) -> bool;

	// Search - find synonyms
	fn search(&self, pat: &str) -> Option<usize>;
	fn locate(&self, pat: &str) -> Option<usize>;

	// Replace synonyms
	fn substitute(&self, from: &str, to: &str) -> String;
	fn swap(&self, from: &str, to: &str) -> String;
}

impl ScriptStrExt for &str {
	fn first(&self) -> String {
		self.chars().next().map(|c| c.to_string()).unwrap_or_default()
	}
	fn head(&self) -> String { self.first() }
	fn start(&self) -> String { self.first() }
	fn begin(&self) -> String { self.first() }

	fn last(&self) -> String {
		self.chars().last().map(|c| c.to_string()).unwrap_or_default()
	}
	fn tail(&self) -> String { self.last() }
	fn end(&self) -> String { self.last() }

	fn reverse(&self) -> String {
		self.chars().rev().collect()
	}

	fn size(&self) -> usize { self.len() }
	fn length(&self) -> usize { self.len() }

	fn upper(&self) -> String { self.to_uppercase() }
	fn to_upper(&self) -> String { self.to_uppercase() }
	fn toUpper(&self) -> String { self.to_uppercase() }
	fn uppercase(&self) -> String { self.to_uppercase() }

	fn lower(&self) -> String { self.to_lowercase() }
	fn to_lower(&self) -> String { self.to_lowercase() }
	fn toLower(&self) -> String { self.to_lowercase() }
	fn lowercase(&self) -> String { self.to_lowercase() }

	fn includes(&self, pat: &str) -> bool { self.contains(pat) }
	fn has(&self, pat: &str) -> bool { self.contains(pat) }
	fn holds(&self, pat: &str) -> bool { self.contains(pat) }

	fn search(&self, pat: &str) -> Option<usize> { self.find(pat) }
	fn locate(&self, pat: &str) -> Option<usize> { self.find(pat) }

	fn substitute(&self, from: &str, to: &str) -> String { self.replace(from, to) }
	fn swap(&self, from: &str, to: &str) -> String { self.replace(from, to) }
}

#[allow(dead_code)]
fn __debug_string<T: std::fmt::Debug>(x: &T) -> String {
	format!("{:?}", x)
}
