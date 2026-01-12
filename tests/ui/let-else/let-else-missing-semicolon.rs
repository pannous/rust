//@ check-pass
// Test that semicolons are inferred after let-else statements

fn main() {
    let Some(x) = Some(1) else {
        return;
    }
    let _ = "";
    let Some(x) = Some(1) else {
        panic!();
    }
}
