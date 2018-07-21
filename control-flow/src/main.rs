fn main() {
    // If else
    let number: u8 = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Bool statement
    // error
    // if number {
    //     println!("number was three");
    // }

    // !=
    if number != 0 {
        println!("number was something other than zero");
    }

    // If else if else
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4, 3");
    }

    // Use let
    let condition = true;
    let number = if condition {
        5
    } else {
        6
        // "six" // error type
    };
    println!("The value of number is: {}", number);

    // Loop and While
    // loop {
    //     println!("again!");
    // }
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");

    // Use For
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // Use For rev
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
