use core::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    let _secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();

    println!("Guess the number");
    println!("Please input the number");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please write a number!");

    println!("You guessed: {}", guess);
    match guess.cmp(&_secret_number) {
        Ordering::Less => {println!("The correct answer was {}",&_secret_number);println!("Too small");},
        Ordering::Greater => {println!("The correct answer was {}",&_secret_number);println!("Too big");},
        Ordering::Equal => println!("You win!"),
    }
}
