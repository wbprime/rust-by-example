fn main() {
    let idx = 6;

    let on_idx = if idx % 3 == 0 { idx * 100 } else { idx };

    println!("on_idx {}", on_idx);

    let mut lidx = 0;
    'outer: loop {
        if lidx == 100 {
            println!("Outer loop {}", lidx);
            lidx = lidx + 10;
            continue;
        }

        loop {
            lidx = lidx + 1;

            if lidx % 2 == 0 {
                println!("Inner loop {}", lidx);
            }
            if lidx % 10 == 0 {
                break;
            }

            if lidx >= 10000 {
                break 'outer;
            }
        }
    }

    for each_i in 1..1000 {
        if (each_i % 2 == 0) {
            println!("For {}", each_i);
        }
    }

    let names = vec!["Zhao", "Qian", "Sun", "Li", "Zhou", "Wu", "Zheng", "Wang"];
    for each_name in names.into_iter() {
        println!("Name {}", each_name);
    }
}
