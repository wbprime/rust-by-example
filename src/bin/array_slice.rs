use std::mem;

fn slicing_info(s: &[i32]) {
    println!("First value is {}", s[0]);
    println!("Length: {}", s.len());
}

fn main() {
    let xs: [i32; 5] = [1, 3, 5, 7, 9];
    let xs2: [i32; 10] = [100; 10];

    slicing_info(&xs2);
    slicing_info(&xs);
    slicing_info(&xs[1..4]);

    println!("Memory size {}", mem::size_of_val(&xs));
}
