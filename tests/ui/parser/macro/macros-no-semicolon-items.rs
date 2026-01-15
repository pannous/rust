macro_rules! foo()  //~ ERROR macros must contain at least one rule

macro_rules! bar {
    ($($tokens:tt)*) => {}
}

bar!(
    blah
    blah
    blah
)

fn main() {
}
