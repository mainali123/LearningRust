// Guessing Game

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to Guess the number game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("{secret_number}");

    let mut guesses_limit = 10;

    while guesses_limit > 1 {
        println!("Please enter your guess ({} guess remaining): ", guesses_limit);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = guess.trim().parse().expect("Please type a number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Equal => { println!("You have guessed correctly!"); break; },
            Ordering::Greater => println!("Too Big"),
        }
        guesses_limit -= 1;
    }
}
