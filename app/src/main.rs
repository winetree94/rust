use std::io;

fn main() {
    println!("Hello, world!");
    println!("type your input");
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("not string");
    let number: i32 = str.trim().parse().expect("not number");
    println!("number parsed: {number}");
}
