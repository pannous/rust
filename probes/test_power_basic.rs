fn main() {
    // Test power operator
    let result = 3 ** 2;
    println!("3 ** 2 = {}", result);
    eq!(3 ** 2, 9);

    // Const eval works correctly
    const A: i32 = 2 ** 3;
    eq!(A, 8);

    println!("Power operator tests passed!");
}
