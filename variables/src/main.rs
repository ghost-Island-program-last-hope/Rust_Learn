fn main() {
    // Immutable
    // let x = 5; // error
    let mut x = 5;
    println!("x: {}", x);
    x = 6;
    println!("x: {}", x);

    // Constants
    const Y: u8 = 8;
    println!("Y: {}", Y);

    // Shadowing
    let i = 5;
    let i = i + 1;
    let i = i * 2;
    println!("i: {}", i);

    // Shadowing - mismatched types
    let space = "   ";
    // space = space.len();
    let space: usize = space.len();
    println!("space len: {}", space);
}
