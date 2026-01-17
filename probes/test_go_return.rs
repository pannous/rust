#!/usr/bin/env rust
# Go-style return type annotation: no -> required

def add(a: i32, b: i32) i32 {
	return a + b
}

def meaning() i32 {
	42
}

def greet() String {
	"Hello".to_string()
}

# Tuple return type
def pair() (i32, i32) {
	(1, 2)
}

# Generic return type
def identity<T>(x: T) T {
	x
}

# Option return type
def maybe(b: bool) Option<i32> {
	if b { Some(42) } else { None }
}

# Verify functions work
def main() {
	assert_eq!(add(2, 3), 5)
	assert_eq!(meaning(), 42)
	assert_eq!(greet(), "Hello")
	assert_eq!(pair(), (1, 2))
	assert_eq!(identity(42), 42)
	assert_eq!(maybe(true), Some(42))
	println!("Go-style return types work!")
}
