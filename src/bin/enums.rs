enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn onWebEvent(e: WebEvent) {
    match e {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        WebEvent::KeyPress(c) => println!("Key {} pressed", c),
        WebEvent::Paste(s) => println!("String {} pasted", s),
        WebEvent::Click { x, y } => println!("Clicked at {},{}", x, y),
    }
}

fn main() {
    let e1 = WebEvent::PageLoad;
    let e2 = WebEvent::PageUnload;
    let e3 = WebEvent::KeyPress('K');
    let e4 = WebEvent::Paste("text".to_owned());
    let e5 = WebEvent::Click { x: 100, y: 200 };

    onWebEvent(e1);
    onWebEvent(e2);
    onWebEvent(e3);
    onWebEvent(e4);
    onWebEvent(e5);
}
