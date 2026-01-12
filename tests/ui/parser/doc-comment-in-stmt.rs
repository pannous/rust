//@ run-rustfix
#![allow(unused)]
fn foo() -> bool { //~ ERROR mismatched types
    false
    //!self.allow_ty_infer()
    //~^ ERROR expected outer doc comment
    //~| ERROR found a documentation comment
}

fn bar() -> bool { //~ ERROR mismatched types
    false
    /*! bar */
    //~^ ERROR expected outer doc comment
    //~| ERROR found a documentation comment
}

fn baz() -> i32 {
    1 /** baz */
    //~^ ERROR expected one of `.`, `;`, `?`, `}`, or an operator
}

fn quux() -> i32 {
    2 /// quux
    //~^ ERROR expected one of `.`, `;`, `?`, `}`, or an operator
}

fn main() {
    let x = 0;
    let y = x.max(1) //!foo
    //~^ ERROR expected one of `.`, `;`, `?`, `else`, or an operator
        .min(2);
}
