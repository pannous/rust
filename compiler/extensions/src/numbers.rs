// Math constants and numeric utilities for script mode.

use std::cell::Cell;
use std::time::{SystemTime, UNIX_EPOCH};


#[allow(nonstandard_style)]
#[allow(dead_code)]
pub const tau: f64 = std::f64::consts::TAU;
#[allow(nonstandard_style)]
#[allow(dead_code)]
pub const pi: f64 = std::f64::consts::PI;
#[allow(nonstandard_style)]
#[allow(dead_code)]
pub const τ: f64 = std::f64::consts::TAU;
#[allow(nonstandard_style)]
#[allow(dead_code)]
pub const π: f64 = std::f64::consts::PI;


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



// Small thread-local PRNG (xorshift64* like) for deterministic, seedable randomness
// kept local to this file to avoid adding an external dependency.

thread_local! {
    static RNG_STATE: Cell<u64> = Cell::new(0);
}

#[allow(dead_code)]
pub fn seed_random(seed: u64) {
	RNG_STATE.with(|s| s.set(seed));
}

fn random64() -> u64 {
	RNG_STATE.with(|s| {
		let mut x = s.get();
		if x == 0 {
			// Initialize with multiple entropy sources mixed together
			let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;

			// Get process ID as entropy
			let pid = std::process::id() as u64;

			// Get thread ID as entropy (using thread::current().id() hash)
			let thread_id = {
				use std::collections::hash_map::DefaultHasher;
				use std::hash::{Hash, Hasher};
				let mut hasher = DefaultHasher::new();
				std::thread::current().id().hash(&mut hasher);
				hasher.finish()
			};

			// Use stack address as additional entropy source
			let stack_addr = {
				let dummy: u8 = 0;
				&dummy as *const u8 as usize as u64
			};

			// Mix all entropy sources together using XOR and rotation
			x = 0x9E3779B97F4A7C15u64 ^ now;
			x ^= pid.rotate_left(17);
			x ^= thread_id.rotate_left(31);
			x ^= stack_addr.rotate_left(47);
		}
		// xorshift64* variant
		x ^= x >> 12;
		x ^= x << 25;
		x ^= x >> 27;
		let out = x.wrapping_mul(2685821657736338717u64);
		s.set(x);
		out
	})
}

#[allow(dead_code)]
pub fn random() -> f64 {
	// use top 53 bits to create a double in [0,1)
	let v = (random64() >> 11) as u64;
	(v as f64) / ((1u64 << 53) as f64)
}

#[allow(dead_code)]
pub fn rand_index(bound: usize) -> usize {
	if bound == 0 { 0 } else { (random64() as usize) % bound }
}

#[allow(dead_code)]
pub fn randint(from: usize, to: usize) -> usize {
	if from == to { 0 } else { ((random64() as usize) % (to-from)) + from}
}
