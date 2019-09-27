fn main() {
    let idx = 10;
    let l1 = match idx {
        1 => 0,
        2 | 3 | 4 | 5 | 6 => 1,
        7...10 => 2,
        _ => 3,
    };
    println!("Matched {}", l1);

    let r1 = &l1;
    match r1 {
        &l1 => println!("l1 got {:?}", l1),
    }

    match r1 {
        ref l2 => println!("l2 got {:?}", l2),
    }
}
