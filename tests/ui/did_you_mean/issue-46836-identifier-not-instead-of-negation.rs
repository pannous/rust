// Note: In this fork, `not` is a valid operator (alias for `!`),
// so most of these expressions now compile successfully.

fn gratitude() {
    let for_you = false;
    if not for_you {
        println!("I couldn't");
    }
}

fn qualification() {
    let the_worst = true;
    while not the_worst {
        println!("still pretty bad");
    }
}

fn should_we() {
    let not = true;
    if not  // lack of braces is [sic]
        println!("Then when?");
    //~^ ERROR expected `{`, found `;`
}

fn sleepy() {
    let _resource = not 2;
}

fn main() {
    let be_smothered_out_before = true;
    let _young_souls = not be_smothered_out_before;
}
