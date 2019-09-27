use std::mem;

fn main() {
    let ai8: i8 = 1;
    let ai16: i16 = 1234;
    let ai32: i32 = 1234;
    let ai64: i64 = 1234;
    let ai128: i128 = 1234;
    let au8: u8 = 1;
    let au16: u16 = 1234u16;
    let au32: u32 = 1234;
    let au64: u64 = 1234;
    let au128: u128 = 1234_000_000u128;

    let abool = true;

    let af32: f32 = 19.0;
    let af64: f64 = 1999.0_000_0000_0;

    let achar = 'C';

    println!("i8 =>{}", ai8);
    println!("i16 =>{}", ai16);
    println!("i32 =>{}", ai32);
    println!("i64 =>{}", ai64);
    println!("i128 =>{}", ai128);
    println!("u8 =>{}", au8);
    println!("u16 =>{}", au16);
    println!("u32 =>{}", au32);
    println!("u64 =>{}", au64);
    println!("u128 =>{}", au128);
    println!("bool =>{}", abool);
    println!("f32 =>{}", af32);
    println!("f64 =>{}", af64);
    println!("char =>{}", achar);

    println!("Size of i8 =>{}", mem::size_of_val(&ai8));
    println!("Size of i16 =>{}", mem::size_of_val(&ai16));
    println!("Size of i32 =>{}", mem::size_of_val(&ai32));
    println!("Size of i64 =>{}", mem::size_of_val(&ai64));
    println!("Size of i128 =>{}", mem::size_of_val(&ai128));
    println!("Size of u8 =>{}", mem::size_of_val(&au8));
    println!("Size of u16 =>{}", mem::size_of_val(&au16));
    println!("Size of u32 =>{}", mem::size_of_val(&au32));
    println!("Size of u64 =>{}", mem::size_of_val(&au64));
    println!("Size of u128 =>{}", mem::size_of_val(&au128));
    println!("Size of bool =>{}", mem::size_of_val(&abool));
    println!("Size of f32 =>{}", mem::size_of_val(&af32));
    println!("Size of f64 =>{}", mem::size_of_val(&af64));
    println!("Size of char =>{}", mem::size_of_val(&achar));

    let bi8 = 256u64 as i8;
    println!("As i8 =>{}", bi8);
    println!("Size of as i8 =>{}", mem::size_of_val(&bi8));
}
