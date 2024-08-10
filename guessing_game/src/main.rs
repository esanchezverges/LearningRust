use core::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    let _secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number");

    loop {
        let mut guess = String::new();
        println!("Please input the number");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&_secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal =>{
                println!("You win!");
                break;
            } ,
        }
    }
}
