#!/usr/bin/env rust
class Point {
    x int
    y int
}

fn (p Point) sum() int {
    return p.x + p.y
}

fn (p Point) display() {
    put!("x: %d, y: %d\n", p.x, p.y)
}

fn main() {
    t := Point{x: 3, y: 4}
    t.display()
    put!("Sum: %d\n", t.sum())
}