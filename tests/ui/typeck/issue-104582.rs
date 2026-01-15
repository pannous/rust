fn main(){
    let my_var: String(String?);
    //~^ ERROR: parenthesized type parameters may only be used with a `Fn` trait
    //~| ERROR: struct takes 0 generic arguments
    // Note: String? is now valid syntax for Option<String>
}
