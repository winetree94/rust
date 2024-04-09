fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("the value of number is: {number}");

    // 표현식이므로, 변수 표현식으로 값을 바로 할당 가능
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("the result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("end count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("liftoff!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}")
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("liftoff!");
}
