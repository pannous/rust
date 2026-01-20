#!/usr/bin/env rust

#[derive(Clone, Copy)]
struct Tee{
	x:int,
	y:int
}

fn sum(t:Tee) -> int{ return t.x + t.y }
fn display(t:Tee){ put!("x:", t.x, "y:", t.y); }

fn tee(x:int, y:int) -> Tee { return Tee{x, y} }

fn test_struct() {
	let t = tee(3, 4)
	// t := tee(3, 4)
	// t.display()
	display(t)
	// put!("Sum:", t.sum())
	put!("Sum:", sum(t));
}

fn main() {
	test_struct();
}