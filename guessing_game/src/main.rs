use std::io;
use rand::prelude::*;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let secret_number = rand::rng().random_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Smaller"),
            std::cmp::Ordering::Greater => println!("Greater"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
