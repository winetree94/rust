fn greeting() {
    println!("Hello, World!")
}

fn snake_case() {
    println!("snake case is recommended");
}

fn another_function(x: i32) {
    println!("the value of x is: {x}");
}

fn expression() {
    let y = {
        let x = 3;
        x + 1;
    };
    // println!("the value of y is: {:?}", y);
}

fn five() -> i32 {
    5
}

fn main() {
    greeting();
    snake_case();
    another_function(32);
    expression();
    let x = five();
    println!("the value of x is: {x}")
}
