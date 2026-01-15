enum Hey<A, B> {
    A(A),
    B(B),
}

struct Foo {
    bar: Option<i32>,
}

fn f() {}

fn a() -> Option<()> {
    // Note: this now compiles due to auto-wrapping from () to Some(())
    while false {
        f();
    }
    // returns Some(()) implicitly
}

fn b() -> Result<(), ()> {
    f()
    //~^ ERROR mismatched types
    //~| HELP try adding an expression
}

fn c() -> Option<()> {
    // Note: this now compiles due to auto-wrapping from () to Some(())
    for _ in [1, 2] {
        f();
    }
    // returns Some(()) implicitly
}

fn d() -> Option<()> {
    // Note: c()? returns () which now auto-wraps to Some(())
    c()?
}

fn main() {
    // Note: while {} returns () which now auto-wraps to Some(())
    let _: Option<()> = while false {};
    let _: Option<()> = {
        // Note: this now compiles due to auto-wrapping
        while false {}
    };
    let _: Result<i32, i32> = 1;
    //~^ ERROR mismatched types
    //~| HELP try wrapping
    // Note: `let _: Option<i32> = 1;` now compiles due to auto-wrapping
    let _: Option<i32> = 1;
    let _: Hey<i32, i32> = 1;
    //~^ ERROR mismatched types
    //~| HELP try wrapping
    let _: Hey<i32, bool> = false;
    //~^ ERROR mismatched types
    //~| HELP try wrapping
    // Note: `let _ = Foo { bar };` now compiles due to auto-wrapping field
    let bar = 1i32;
    let _ = Foo { bar };
}

enum A {
    B { b: B },
}

struct A2(B);

enum B {
    Fst,
    Snd,
}

fn foo() {
    let a: A = B::Fst;
    //~^ ERROR mismatched types
    //~| HELP try wrapping
}

fn bar() {
    let a: A2 = B::Fst;
    //~^ ERROR mismatched types
    //~| HELP try wrapping
}
