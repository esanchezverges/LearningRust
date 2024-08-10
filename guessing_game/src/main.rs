use std::io;
use rand::Rng;

fn main() {
    let _secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number");
    println!("Please input the number");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    if guess == _secret_number.to_string() {
        println!("You guessed right")
    }
    else {
        println!("You guessed wrong, the correct answer was {}",_secret_number);
    }
}
