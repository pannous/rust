//@ check-pass
// Note: The original test used `not 1` to test macro recovery,
// but this fork supports `not` as an alias for `!`, so `not 1` is now valid.

macro_rules! dont_recover_here {
    ($e:expr) => {};
    (xyzzy $a:literal) => {};
}

// Using a nonsense keyword that won't be parsed as an expression
dont_recover_here! { xyzzy 1 }

fn main() {}
