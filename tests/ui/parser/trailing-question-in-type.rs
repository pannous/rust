//@ run-pass
// Test that T? is valid syntax for Option<T>

fn foo() -> i32? {
    let x: i32? = Some(1);
    x
}

fn main() {
    let _: Option<i32> = foo();
    let val: i32? = Some(42);
    assert_eq!(val, Some(42));
}
