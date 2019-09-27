use std::mem;

fn map_on_fn_once<F>(f: F) -> i32
where
    F: FnOnce(i32) -> i32,
{
    f(11)
}

fn map_on_fn<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(22)
}

// fn map_on_fnMut<F>(f: F) -> i32
// where
//     F: FnMut(i32) -> i32,
// {
//     f(33)
// }

fn apply_on_fn_once<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply_on_fn<F>(f: F)
where
    F: Fn(),
{
    f();
}

// fn apply_on_fnMut<F>(f: F)
// where
//     F: FnMut(),
// {
//     f();
// }

fn main() {
    let color = "green";

    let print = || println!("`color` is {}", color);

    print();
    print();

    let mut count = 0;
    let mut increasing = || {
        count += 1;
        println!("Count is {}", count);
    };

    increasing();
    increasing();

    let disposable = Box::new(100);
    let droping = || {
        println!("`Droping` {:?}", disposable);
        mem::drop(disposable);
    };

    droping();

    let msg = "Hello";
    // let mut msg2 = "Hello".to_owned();

    let greeting1 = || {
        println!("{}", msg);
    };
    // let greeting2 = || {
    //     msg2.push_str("!!!");
    //     println!("{}", msg2);
    // };
    let greeting3 = move || {
        println!("{}", msg);
    };

    apply_on_fn(greeting1);
    // apply_on_fnMut(greeting2);
    apply_on_fn_once(greeting3);

    let x1 = 10;
    // let mut x2 = 10;
    let x3 = 10;

    let doubling1 = |x| x * x1;
    // let doubling2 = |x| {
    //     x2 += 1;
    //     x * x2
    // };
    let doubling3 = move |x| x * x3;

    map_on_fn(doubling1);
    // map_on_fnMut(doubling2);
    map_on_fn_once(doubling3);
}
