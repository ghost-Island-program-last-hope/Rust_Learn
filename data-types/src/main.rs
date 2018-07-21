fn main() {
    // Compound types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // destructure
    let (_x, _y, _z) = tup;
    println!("{}", _x);

    println!("tup-0: {}", tup.0);
    println!("tup-1: {}", tup.1);
    println!("tup-2: {}", tup.2);

    // Array
    let arr = [1, 2, 3, 4, 5];
    println!("{}", arr[0]);
    let index = 3;
    println!("The value of element is: {}", arr[index]);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("{}", months[1]);
}
