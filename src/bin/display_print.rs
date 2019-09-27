use std::fmt;

struct Single(i32);

impl fmt::Display for Single {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Minmax(i64, i64);

impl fmt::Display for Minmax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Min: {}, Max: {}", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point X({}) Y({})", self.x, self.y)
    }
}

#[derive(Debug)]
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[");

        let v = &self.0;

        for (i, v) in v.iter().enumerate() {
            if (i != 0) {
                write!(f, ", ");
            }

            write!(f, "[{}] => {}", i, v);
        }

        write!(f, "]")
    }
}

fn main() {
    let single = Single(100);

    println!("Single {}", single);

    let minmax = Minmax(100, 1000);

    println!("Minmax {}", minmax);

    let point = Point2D {
        x: 100.0,
        y: 1000.0,
    };

    println!("Point2D DISPLAY {}", point);
    println!("Point2D DEBUG {:?}", point);

    let list = List(vec![1, 4, 7, 10, 56]);
    println!("List DISPLAY {}", list);
    println!("List DEBUG {:?}", list);
}
