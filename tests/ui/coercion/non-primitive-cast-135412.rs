//@ run-rustfix

use std::sync::Arc;

fn main() {
    // Note: `7u32 as Option<_>` now works due to auto-wrapping to Some(7)
    let _: Option<u32> = 7u32 as Option<_>;
    let _ = "String" as Arc<str>;
    //~^ ERROR non-primitive cast: `&'static str` as `Arc<str>`
}
