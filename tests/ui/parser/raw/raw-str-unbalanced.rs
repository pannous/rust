static s: &'static str =
    r#""## //~ ERROR too many `#` when terminating raw string
;

static s2: &'static str =
    r#"
      "#### //~ ERROR expected one of `!` or `[`, found `#`
;

const A: &'static str = r""

// Test
#[test]
fn test() {}

const B: &'static str = r""##

// Test
#[test]
fn test2() {}

fn main() {}
