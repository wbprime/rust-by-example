struct Nil;

struct Pair(i32, i64);

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn rect_area(r: &Rectangle) -> f64 {
    let Point { x: thex1, y: they1 } = r.p1;
    let Point { x: thex2, y: they2 } = r.p2;

    let w = thex2 - thex1;
    let h = they2 - they1;

    w.abs() * h.abs()
}

fn square(p: Point, v: f64) -> Rectangle {
    let x2 = p.x + v;
    let y2 = p.y + v;
    Rectangle {
        p1: p,
        p2: Point { x: x2, y: y2 },
    }
}

fn main() {
    let x = 100.0;
    let y = 200.1;
    let p = Point { x, y };
    let p2 = Point { x, y };

    let r = Rectangle {
        p1: Point { x: 1.1, y: 2.2 },
        p2: p,
    };

    let Point { x: thex, y: they } = p2;

    println!("Rectangle {:?}", r);
    println!("Rectangle area {:?}", rect_area(&r));
    println!("Point {},{}", thex, they);
    println!("Rectangle returned {:?}", square(p2, 30.0));
}
