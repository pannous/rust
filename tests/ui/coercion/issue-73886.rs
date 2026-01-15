fn main() {
    let _ = &&[0] as &[_];
    //~^ ERROR non-primitive cast: `&&[i32; 1]` as `&[_]`
    // Note: `7u32 as Option<_>` now works due to auto-wrapping to Some(7)
    let _: Option<u32> = 7u32 as Option<_>;
}
