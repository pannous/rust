fn main() {
    println!("%.*3$s %s!\n", "Hello,", "World", 4); //~ ERROR multiple unused formatting arguments
    println!("%1$*2$.*3$f", 123.456); //~ ERROR never used
    println!(r###"%.*3$s
        %s!\n
"###, "Hello,", "World", 4);
    //~^ ERROR multiple unused formatting arguments
    // correctly account for raw strings in inline suggestions

    // Note: printf-style %f is now supported, so mixing {} and %f works
    println!("{} %f", "one", 2.0); // Now compiles: {} takes "one", %f takes 2.0

    println!("Hi there, $NAME.", NAME="Tim"); //~ ERROR never used
    println!("$1 $0 $$ $NAME", 1, 2, NAME=3);
    //~^ ERROR multiple unused formatting arguments
}
