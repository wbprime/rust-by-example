use std::convert::From;
use std::convert::Into;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
struct Counter {
    v: u32,
}

impl Into<String> for Counter {
    fn into(self) -> String {
        format!("{}", self.v)
    }
}

impl From<u32> for Counter {
    fn from(v: u32) -> Self {
        Counter { v }
    }
}

impl Display for Counter {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "{}", self.v)
    }
}

// impl FromStr for Counter {
//     type Err = ();

//     fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
//         s.<u32>parse().map(|v| Counter { v })
//     }
// }

fn main() {
    let c = Counter::from(30);

    println!("Counter {:?}", c);
    println!("Counter {}", c);

    let s: String = Counter::into(c);

    println!("Stringify {}", s);
}
