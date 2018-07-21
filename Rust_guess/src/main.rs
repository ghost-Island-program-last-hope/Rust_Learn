extern crate rand; //把rand套件載入，載入後可以使用random

use rand::Rng; //使用rnad套件中的Rng函式
use std::cmp::Ordering; //使用比較的函式
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101); //實作random

    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); //mut可以將變數變為不固定

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() { //將guess從字串變為數字
            Ok(num) => num,
            Err(_) => {
                println!("{} is not a number", guess.trim());
                continue; //重新進入loop
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) { //實作cmp函式
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
