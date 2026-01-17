#![feature(cfg_select)]
#![crate_type = "lib"]

// Check that parse errors in arms that are not selected are still reported.
// Note: ++ is now valid syntax in this fork, so we use actual syntax errors.

fn print() {
    println!(cfg_select! {
        false => { 1 @ 2 }
        //~^ ERROR macro expansion ignores
        _ => { "not unix" }
    });
}

cfg_select! {
    false => { fn foo() { 1 @ 2 } }
    //~^ ERROR expected one of
    _ => {}
}
