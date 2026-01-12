//@ check-pass
// Test that macro invocations with ambiguous content still work

macro_rules! please_recover {
    ($a:expr) => {};
}

please_recover! { not 1 }

fn main() {}
