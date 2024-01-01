use std::{cmp::Ordering,io};
use rand::Rng;

pub fn main() {
    println!("Guess the number between 1 to 100");
    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Input your guess!");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Guess greater than secret"),
            Ordering::Less => println!("Guess less than secret"),
            Ordering::Equal => {
                println!("Congratz!");
                break;
            }
        }
    }
}