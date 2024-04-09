const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
const MY_NAME: &str = "winetree94";

fn main2() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("Hours: {THREE_HOURS_IN_SECONDS}");

    println!("my name is: {MY_NAME}");
}

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x isn the inner scope is: {x}");

        let spaces = "    ";
        let spaces = spaces.len();
        println!("leng is : {spaces}");
    }

    println!("The value of x is: {x}");

    // println!("the usize is {usize}");
    // println!("the isize is {isize}");
}

fn main3() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}
