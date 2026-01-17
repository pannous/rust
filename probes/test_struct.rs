#!/usr/bin/env rust

// needs main()
struct Tee{
	x:int,
	y:int
}

fn sum(t:Tee) -> int{ return t.x + t.y }
fn display(t:Tee){ put!("x:", t.x, "y:", t.y) }

fn tee(x:int, y:int) -> Tee { return Tee{x, y} }

fn test_struct() {
	t := tee(3, 4)
	t.display()
	put!("Sum:", t.sum())
}

fn main() {
	test_struct()
}