use warp::wasp_parser::WaspParser;
fn main() {
    let cases = [
        "square 1+2",
        "1+square 2+3",
        "1+square(2+3)",
        "square 2+3",
    ];
    for case in cases {
        let mut parser = WaspParser::new(case);
        let node = parser.parse();
        println!("{:?} => {:?}", case, node);
    }
}
