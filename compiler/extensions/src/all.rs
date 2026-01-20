// All script mode extensions in one file.
//
// This file is parsed at compile time by script_harness.rs and injected
// as a module into scripts. Items are made public so they can be re-exported.

// ============================================================================
// TRUTHY TRAIT
// ============================================================================

#[allow(dead_code)]
pub trait Truthy {
	fn is_truthy(&self) -> bool;
}

impl Truthy for bool {
	fn is_truthy(&self) -> bool { *self }
}

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

impl Truthy for f32 { fn is_truthy(&self) -> bool { *self != 0.0 } }
impl Truthy for f64 { fn is_truthy(&self) -> bool { *self != 0.0 } }

impl Truthy for &str { fn is_truthy(&self) -> bool { !self.is_empty() } }
impl Truthy for String { fn is_truthy(&self) -> bool { !self.is_empty() } }

impl<T> Truthy for Vec<T> { fn is_truthy(&self) -> bool { !self.is_empty() } }
impl<T> Truthy for Option<T> { fn is_truthy(&self) -> bool { self.is_some() } }

// ============================================================================
// STRING EXTENSIONS
// ============================================================================

#[allow(dead_code)]
pub trait ScriptStrExt {
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
}

impl ScriptStrExt for &str { // todo rename to StringExtensions
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
}

#[allow(dead_code)]
pub fn __debug_string<T: std::fmt::Debug>(x: &T) -> String {
	format!("{:?}", x)
}

// ============================================================================
// LIST/SLICE EXTENSIONS
// ============================================================================

#[allow(dead_code)]
pub trait ScriptSliceExt<T: Clone> {
	fn mapped<U, F: Fn(T) -> U>(&self, f: F) -> Vec<U>;
	fn apply<U, F: Fn(T) -> U>(&self, f: F) -> Vec<U>;
	fn transform<U, F: Fn(T) -> U>(&self, f: F) -> Vec<U>;
	fn convert<U, F: Fn(T) -> U>(&self, f: F) -> Vec<U>;
	fn filtered<F: Fn(&T) -> bool>(&self, f: F) -> Vec<T>;
	fn select<F: Fn(&T) -> bool>(&self, f: F) -> Vec<T>;
	fn chose<F: Fn(&T) -> bool>(&self, f: F) -> Vec<T>;
	fn that<F: Fn(&T) -> bool>(&self, f: F) -> Vec<T>;
	fn which<F: Fn(&T) -> bool>(&self, f: F) -> Vec<T>;
	fn first_cloned(&self) -> Option<T>;
	fn shift(&self) -> Option<T>;
	fn pairs(&self) -> Vec<(usize, T)>;
}

impl<T: Clone, S: AsRef<[T]>> ScriptSliceExt<T> for S {
	fn mapped<U, F: Fn(T) -> U>(&self, f: F) -> Vec<U> { self.as_ref().iter().cloned().map(f).collect() }
	fn apply<U, F: Fn(T) -> U>(&self, f: F) -> Vec<U> { self.mapped(f) }
	fn transform<U, F: Fn(T) -> U>(&self, f: F) -> Vec<U> { self.mapped(f) }
	fn convert<U, F: Fn(T) -> U>(&self, f: F) -> Vec<U> { self.mapped(f) }
	fn filtered<F: Fn(&T) -> bool>(&self, f: F) -> Vec<T> { self.as_ref().iter().filter(|x| f(x)).cloned().collect() }
	fn select<F: Fn(&T) -> bool>(&self, f: F) -> Vec<T> { self.filtered(f) }
	fn chose<F: Fn(&T) -> bool>(&self, f: F) -> Vec<T> { self.filtered(f) }
	fn that<F: Fn(&T) -> bool>(&self, f: F) -> Vec<T> { self.filtered(f) }
	fn which<F: Fn(&T) -> bool>(&self, f: F) -> Vec<T> { self.filtered(f) }
	fn first_cloned(&self) -> Option<T> { self.as_ref().first().cloned() }
	fn shift(&self) -> Option<T> { self.first_cloned() }
	fn pairs(&self) -> Vec<(usize, T)> { self.as_ref().iter().cloned().enumerate().collect() }
}

#[allow(dead_code)]
pub fn slice_eq<T: PartialEq, A: AsRef<[T]>, B: AsRef<[T]>>(a: &A, b: &B) -> bool {
	a.as_ref() == b.as_ref()
}

// ============================================================================
// VAL ENUM (Dynamic Typing)
// ============================================================================

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

impl From<&str> for Val { fn from(s: &str) -> Self { Val::Str(s.to_string()) } }
impl From<String> for Val { fn from(s: String) -> Self { Val::Str(s) } }
impl From<i64> for Val { fn from(n: i64) -> Self { Val::Int(n) } }
impl From<i32> for Val { fn from(n: i32) -> Self { Val::Int(n as i64) } }
impl From<f64> for Val { fn from(n: f64) -> Self { Val::Float(n) } }
impl From<f32> for Val { fn from(n: f32) -> Self { Val::Float(n as f64) } }
impl From<bool> for Val { fn from(b: bool) -> Self { Val::Bool(b) } }
impl From<char> for Val { fn from(c: char) -> Self { Val::Str(c.to_string()) } }

impl PartialEq<char> for Val {
	fn eq(&self, other: &char) -> bool {
		match self {
			Val::Str(s) => s.len() == 1 && s.chars().next() == Some(*other),
			_ => false,
		}
	}
}

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

// ============================================================================
// MATH CONSTANTS & FUNCTIONS
// ============================================================================

#[allow(dead_code)]
pub const tau: f64 = std::f64::consts::TAU;
#[allow(dead_code)]
pub const pi: f64 = std::f64::consts::PI;
#[allow(dead_code)]
pub const τ: f64 = std::f64::consts::TAU;
#[allow(dead_code)]
pub const π: f64 = std::f64::consts::PI;

#[allow(dead_code)]
pub fn approx_eq(a: f64, b: f64) -> bool {
	let epsilon = 1e-9_f64;
	(a - b).abs() < epsilon.max(a.abs() * epsilon).max(b.abs() * epsilon)
}

#[allow(dead_code)]
pub fn exit(code: i32) -> ! {
	std::process::exit(code)
}

// ============================================================================
// MACROS (need #[macro_export] to be usable outside module)
// ============================================================================

#[macro_export]
macro_rules! put {
    () => { println!() };
    ($e:expr $(,)?) => { println!("{:?}", $e) };
    ($first:expr, $($rest:expr),+ $(,)?) => {
        print!("{:?}", $first);
        $(print!(" {:?}", $rest);)+
        println!();
    };
}

#[macro_export]
macro_rules! printf {
    ($($arg:tt)*) => { println!($($arg)*) };
}

#[macro_export]
macro_rules! eq {
    ($left:expr, $right:expr) => { assert_eq!($left, $right) };
}

#[macro_export]
macro_rules! eqs {
    ($left:expr, $right:expr) => { assert_eq!(format!("{:?}", $left), $right) };
}

#[macro_export]
macro_rules! seq {
    ($left:expr, $right:expr) => { assert!(slice_eq(&$left, &$right)) };
}

#[macro_export]
macro_rules! s {
    ($e:expr) => { { let __s: String = $e.into(); __s } };
}

#[macro_export]
macro_rules! typeid {
    ($e:expr) => { std::any::type_name_of_val(&$e) };
}

#[macro_export]
macro_rules! exit {
    () => { exit(0) };
    ($code:expr) => { exit($code) };
}
