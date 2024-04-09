fn immutable_string() -> &'static str {
    let mut s = "lkajsdf";
    s = "sdlfkj";
    s
}

fn mutable_string() -> String {
    let mut s = String::from("hello");
    s.push_str(", world!");
    s
}

fn main() {
    println!("Hello, world!");

    println!("immerable_string: {}", immutable_string());
    println!("mutable_string: {}", mutable_string());
}
