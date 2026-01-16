// Test i++ and i-- syntax (desugars to i += 1 and i -= 1)
fn main() {
    let mut i = 0;
    i++;
    assert_eq!(i, 1);

    let mut j = 5;
    j--;
    assert_eq!(j, 4);

    // Field access
    struct Counter { value: i32 }
    let mut c = Counter { value: 0 };
    c.value++;
    assert_eq!(c.value, 1);

    // Array index
    let mut arr = [0, 1, 2];
    arr[0]++;
    assert_eq!(arr[0], 1);

    println!("All increment/decrement tests passed!");
}
