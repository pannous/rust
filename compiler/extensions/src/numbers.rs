// Math constants and numeric utilities for script mode.

#[allow(dead_code)]
pub const tau: f64 = std::f64::consts::TAU;
#[allow(dead_code)]
pub const pi: f64 = std::f64::consts::PI;
#[allow(dead_code)]
pub const τ: f64 = std::f64::consts::TAU;
#[allow(dead_code)]
pub const π: f64 = std::f64::consts::PI;

use std::cell::Cell;

thread_local! {
	static EPSILON: Cell<f64> = Cell::new(1e-6);
}

#[allow(dead_code)]
pub fn set_epsilon(eps: f64) {
	EPSILON.with(|e| e.set(eps));
}

#[allow(dead_code)]
pub fn get_epsilon() -> f64 {
	EPSILON.with(|e| e.get())
}

#[allow(dead_code)]
pub fn approx_eq(a: f64, b: f64) -> bool {
	let epsilon = get_epsilon();
	(a - b).abs() < epsilon.max(a.abs() * epsilon).max(b.abs() * epsilon)
}

#[allow(dead_code)]
pub fn exit(code: i32) -> ! {
	std::process::exit(code)
}
