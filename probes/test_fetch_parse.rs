#!/usr/bin/env rustc
use warp::wasp_parser::WaspParser;
use warp::node::Node;

fn print_node(node: &Node, indent: usize) {
    let pad = "  ".repeat(indent);
    match node.drop_meta() {
        Node::Key(k, op, v) => {
            println!("{}Key({:?}):", pad, op);
            println!("{}  left:", pad);
            print_node(&k, indent + 2);
            println!("{}  right:", pad);
            print_node(&v, indent + 2);
        }
        Node::List(items, _br, _sep) => {
            println!("{}List({} items):", pad, items.len());
            for (i, item) in items.iter().enumerate() {
                println!("{}  [{}]:", pad, i);
                print_node(item, indent + 2);
            }
        }
        Node::Symbol(s) => println!("{}Symbol({})", pad, s),
        Node::Text(s) => println!("{}Text({})", pad, s),
        Node::Number(n) => println!("{}Number({:?})", pad, n),
        other => println!("{}Other: {:?}", pad, other),
    }
}

fn main() {
    // Test the failing expression
    let expr = "x=fetch https://pannous.com/files/test;i=7;x";
    let node = WaspParser::parse(expr);
    println!("Input: {}", expr);
    println!("Structure:");
    print_node(&node, 0);
}
