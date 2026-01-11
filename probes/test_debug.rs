use warp::wasp_parser::parse;
use warp::extensions::print;

fn main() {
    let expr = "global x=1+3.14";
    let node = parse(expr);
    print(&format!("Parsed: {:?}", node));
}
