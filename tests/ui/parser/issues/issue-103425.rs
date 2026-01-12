//@ check-pass
// Test that semicolons are inferred between statements

fn f() -> f32 {
    3
    5.0
}

fn k() -> f32 {
    2_u32
    3_i8
    5.0
}

fn main() {}
