enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        List::Nil
    }

    fn prepend(self, v: u32) -> List {
        List::Cons(v, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            List::Cons(_, ref tail) => 1 + tail.len(),
            List::Nil => 0,
        }
    }

    fn stringfy(&self) -> String {
        match *self {
            List::Cons(cur, ref tail) => format!("{}, {}", cur, tail.stringfy()),
            List::Nil => format!("Nil"),
        }
    }
}

fn main() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    list = list.prepend(4);
    list = list.prepend(5);

    println!("List size: {}", list.len());
    println!("List stringfied as {}", list.stringfy());
}
