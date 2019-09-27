#[derive(Debug)]
struct DebugPritable(i32);

#[derive(Debug)]
struct DebugPerson<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean");

    println!("Hello \n se7en");

    /*
     * first level second level
     */

    let x = 5 + 90 + 5;
    println!("Output result is {}", x);

    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bod");

    println!("{sub}, {verb}, {obj}", obj = "B", sub = "A", verb = "is");

    println!("{} of {:b} know binary", 1, 2);

    println!(
        "{number:<width$} A {number:>width$} B",
        number = 1,
        width = 5
    );
    println!(
        "{number:<0width$} A {number:>0width$} B",
        number = 1,
        width = 5
    );

    let pi = 3.141592;
    println!("Pi is {:2.2}", pi);

    println!("Debug {:?}", DebugPritable(10));

    let aname = "Peter";
    let age = 9;
    let p = DebugPerson { name: aname, age };
    println!("Person {:#?}", p);
}
