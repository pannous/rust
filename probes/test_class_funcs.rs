#!/usr/bin/env rust
class Point {
    x int
    y int
}

func (p Point) sum() int {
    return p.x + p.y
}

func (p Point) display() {
    put!("x: %d, y: %d\n", p.x, p.y)
}

func main() {
    t := Point{x: 3, y: 4}
    t.display()
    put!("Sum: %d\n", t.sum())
}