#!/usr/bin/env rust
use warp::wasp_parser::parse;
use warp::node::Node;
fn main() {
    let code = "{name:String age:i64}";
    let node = parse(code);
    print_node(&node, 0);
}
fn print_node(n: &Node, indent: usize) {
    let prefix = "  ".repeat(indent);
    let inner = n.drop_meta();
    match inner {
        Node::Key(left, op, right) => {
            println!("{}Key({:?}):", prefix, op);
            print_node(left, indent + 1);
            print_node(right, indent + 1);
        }
        Node::Symbol(s) => println!("{}Symbol({})", prefix, s),
        Node::Text(s) => println!("{}Text({})", prefix, s),
        Node::List(items, _bracket, _sep) => {
            println!("{}List[{}]:", prefix, items.len());
            for item in items {
                print_node(item, indent + 1);
            }
        }
        Node::Type { name, body } => {
            println!("{}Type:", prefix);
            println!("{}  name:", prefix);
            print_node(name, indent + 2);
            println!("{}  body:", prefix);
            print_node(body, indent + 2);
        }
        Node::Empty => println!("{}Empty", prefix),
        _ => println!("{}Other: {:?}", prefix, inner),
    }
}
