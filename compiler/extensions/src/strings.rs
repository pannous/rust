// String extension methods for script mode.
//
// Provides convenient string methods with intuitive synonyms.

#[allow(dead_code)]
pub trait StringExtensions {
	fn first(&self) -> String;
	fn head(&self) -> String;
	fn start(&self) -> String;
	fn begin(&self) -> String;
	fn last(&self) -> String;
	fn tail(&self) -> String;
	fn end(&self) -> String;
	fn reverse(&self) -> String;
	fn size(&self) -> usize;
	fn length(&self) -> usize;
	fn upper(&self) -> String;
	fn to_upper(&self) -> String;
	fn toUpper(&self) -> String;
	fn uppercase(&self) -> String;
	fn lower(&self) -> String;
	fn to_lower(&self) -> String;
	fn toLower(&self) -> String;
	fn lowercase(&self) -> String;
	fn includes(&self, pat: &str) -> bool;
	fn has(&self, pat: &str) -> bool;
	fn holds(&self, pat: &str) -> bool;
	fn search(&self, pat: &str) -> Option<usize>;
	fn locate(&self, pat: &str) -> Option<usize>;
	fn substitute(&self, from: &str, to: &str) -> String;
	fn swap(&self, from: &str, to: &str) -> String;
	fn swapy(&self, from: &str, to: &str) -> String;
}

impl StringExtensions for &str {
	fn first(&self) -> String { self.chars().next().map(|c| c.to_string()).unwrap_or_default() }
	fn head(&self) -> String { self.first() }
	fn start(&self) -> String { self.first() }
	fn begin(&self) -> String { self.first() }
	fn last(&self) -> String { self.chars().last().map(|c| c.to_string()).unwrap_or_default() }
	fn tail(&self) -> String { self.last() }
	fn end(&self) -> String { self.last() }
	fn reverse(&self) -> String { self.chars().rev().collect() }
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
	fn swapy(&self, from: &str, to: &str) -> String { self.replace(from, to) }
}

impl StringExtensions for String {
	fn first(&self) -> String { self.as_str().first() }
	fn head(&self) -> String { self.as_str().head() }
	fn start(&self) -> String { self.as_str().start() }
	fn begin(&self) -> String { self.as_str().begin() }
	fn last(&self) -> String { self.as_str().last() }
	fn tail(&self) -> String { self.as_str().tail() }
	fn end(&self) -> String { self.as_str().end() }
	fn reverse(&self) -> String { self.as_str().reverse() }
	fn size(&self) -> usize { self.as_str().size() }
	fn length(&self) -> usize { self.as_str().length() }
	fn upper(&self) -> String { self.as_str().upper() }
	fn to_upper(&self) -> String { self.as_str().to_upper() }
	fn toUpper(&self) -> String { self.as_str().toUpper() }
	fn uppercase(&self) -> String { self.as_str().uppercase() }
	fn lower(&self) -> String { self.as_str().lower() }
	fn to_lower(&self) -> String { self.as_str().to_lower() }
	fn toLower(&self) -> String { self.as_str().toLower() }
	fn lowercase(&self) -> String { self.as_str().lowercase() }
	fn includes(&self, pat: &str) -> bool { self.as_str().includes(pat) }
	fn has(&self, pat: &str) -> bool { self.as_str().has(pat) }
	fn holds(&self, pat: &str) -> bool { self.as_str().holds(pat) }
	fn search(&self, pat: &str) -> Option<usize> { self.as_str().search(pat) }
	fn locate(&self, pat: &str) -> Option<usize> { self.as_str().locate(pat) }
	fn substitute(&self, from: &str, to: &str) -> String { self.as_str().substitute(from, to) }
	fn swap(&self, from: &str, to: &str) -> String { self.as_str().swap(from, to) }
	fn swapy(&self, from: &str, to: &str) -> String { self.as_str().swap(from, to) }
}

#[allow(dead_code)]
pub fn __debug_string<T: std::fmt::Debug>(x: &T) -> String {
	format!("{:?}", x)
}
