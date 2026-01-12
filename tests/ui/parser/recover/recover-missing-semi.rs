fn main() {
    let _: usize = ()
    //~^ ERROR mismatched types
    let _ = 3;
}

fn foo() -> usize {
    let _: usize = ()
    //~^ ERROR mismatched types
    return 3;
}
