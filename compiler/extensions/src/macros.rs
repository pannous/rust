// Convenience macros for script mode.
// Note: Inner attributes (#![...]) cannot be used here as this file
// is concatenated with others and parsed as a fragment.

// Print values with Debug formatting, Python-style multiple args.

#[allow(unused)]
macro_rules! put {
    () => { println!() };
    ($e:expr $(,)?) => { println!("{:?}", $e) };
    ($first:expr, $($rest:expr),+ $(,)?) => {
        print!("{:?}", $first);
        $(print!(" {:?}", $rest);)+
        println!();
    };
}

// Printf-style printing with format string.
#[allow(unused)]
macro_rules! printf {
    ($($arg:tt)*) => { println!($($arg)*) };
}

// Assert equality shorthand.
#[allow(unused)]
macro_rules! eq {
    ($left:expr, $right:expr) => { assert_eq!($left, $right) };
}

// Assert equality comparing Debug format of left to right.
#[allow(unused)]
macro_rules! eqs {
    ($left:expr, $right:expr) => { assert_eq!(format!("{:?}", $left), $right) };
}

// Slice equality using slice_eq helper.
#[allow(unused)]
macro_rules! seq {
    ($left:expr, $right:expr) => { assert!(slice_eq(&$left, &$right)) };
}

// Convert expression to String.
#[allow(unused)]
macro_rules! s {
    ($e:expr) => { { let __s: String = $e.into(); __s } };
}

// Get the type name of an expression.
#[allow(unused)]
macro_rules! typeid {
    ($e:expr) => { std::any::type_name_of_val(&$e) };
}

// Exit macro with optional exit code.
#[allow(unused)]
macro_rules! exit {
    () => { exit(0) };
    ($code:expr) => { exit($code) };
}

// Internal macro for truthy if statements.
#[allow(unused)]
macro_rules! __if {
    ($cond:expr ; $($rest:tt)*) => { if (&$cond).is_truthy() $($rest)* };
}

// Internal macro for statement parsing.
#[allow(unused)]
macro_rules! __stmt {
    ($($t:tt)*) => { $($t)*; };
}
