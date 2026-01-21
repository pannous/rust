// Math constants and numeric utilities for script mode.

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
