//@ check-pass
// Note: In this fork, `and` and `or` are valid operators (aliases for `&&` and `||`),
// so these expressions now compile successfully.

#![allow(dead_code, unused_parens)]

fn main() {}

fn test_and() {
    let a = true;
    let b = false;

    let _ = a and b;

    if a and b {
        println!("both");
    }
}

fn test_or() {
    let a = true;
    let b = false;

    let _ = a or b;

    if a or b {
        println!("both");
    }
}

fn test_and_par() {
    let a = true;
    let b = false;
    if (a and b) {
        println!("both");
    }
}

fn test_or_par() {
    let a = true;
    let b = false;
    if (a or b) {
        println!("both");
    }
}

fn test_while_and() {
    let a = true;
    let b = false;
    while a and b {
        println!("both");
    }
}

fn test_while_or() {
    let a = true;
    let b = false;
    while a or b {
        println!("both");
    }
}
