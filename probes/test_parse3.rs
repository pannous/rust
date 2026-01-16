#!/usr/bin/env rust
use warp::wasp_parser::parse;
use warp::node::Node;
fn main() {
    let code = "fetch https://pannous.com/files/test";
    let node = parse(code);
    print_node(&node, 0);
}

fn print_node(n: &Node, indent: usize) {
    let prefix = "  ".repeat(indent);
    match n {
        Node::Key(left, op, right) => {
            println!("{}Key({:?}):", prefix, op);
            print_node(left, indent + 1);
            print_node(right, indent + 1);
        }
        Node::Symbol(s) => println!("{}Symbol({})", prefix, s),
        Node::Text(s) => println!("{}Text({})", prefix, s),
        Node::List(items, _bracket, _) => {
            println!("{}List:", prefix);
            for item in items {
                print_node(item, indent + 1);
            }
        }
        Node::Error(inner) => {
            println!("{}Error:", prefix);
            print_node(inner, indent + 1);
        }
        _ => println!("{}Other: {:?}", prefix, n),
    }
}
