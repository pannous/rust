//@ check-pass
// Test that semicolons are inferred before item definitions

#![allow(unused_variables, dead_code, unused_imports)]

fn for_struct() {
    let foo = 3
    struct Foo;
}

fn for_union() {
    let foo = 3
    union Foo {
        foo: usize,
    }
}

fn for_enum() {
    let foo = 3
    enum Foo {
        Bar,
    }
}

fn for_fn() {
    let foo = 3
    fn foo() {}
}

fn for_extern() {
    let foo = 3
    extern "C" fn foo() {}
}

fn for_impl() {
    struct Foo;
    let foo = 3
    impl Foo {}
}

fn for_use() {
    let foo = 3
    pub use bar::Bar;
}

fn for_mod() {
    let foo = 3
    mod foo {}
}

fn for_type() {
    let foo = 3
    type Foo = usize;
}

mod bar {
    pub struct Bar;
}

const X: i32 = 123

fn main() {}
