use std::fmt;

fn revert(p: (i32, u64)) -> (u64, i32) {
    (p.1, p.0)
}

struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            // "{:2.4},{:2.4},{:2.4},{:2.4}",
            "({:2.4},{:2.4})\n({:2.4},{:2.4})",
            self.0, self.1, self.2, self.3
        )
    }
}

fn transpose(m: Matrix) -> Matrix {
    Matrix(m.0, m.2, m.1, m.3)
}

fn main() {
    let pair = (100, 200u64);
    let out = revert(pair);

    println!("Reverted {:?}, as {:?}", pair, out);

    let m = Matrix(2.0, 1.0, 4.0, 3.0);
    println!("{}", m);
    println!("{}", transpose(m));
}
