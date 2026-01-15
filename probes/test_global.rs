#!/usr/bin/env rustc
use wasp::is;

#[test]
fn test_global_simple() {
    use std::f64::consts::PI;
    is!("global x=1+π", 1.0 + PI);
}
