fn main() {
    println!("Hello, world!");
    greet("Robby", 7);

    let _x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let result: u32 = turnfn(2, 3);
    println!("{}", result);
}

fn greet(name: &str, date: u8) {
    println!("Hi {}!, Date:{}", name, date);
}

fn turnfn(x: u32, y: u32) -> u32 {
    x * y
}
