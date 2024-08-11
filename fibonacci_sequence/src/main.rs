use std::io::stdin;

fn main() {
    let mut user_input = String::new();

    println!("Fibonacci sequence");
    println!("Input a number to get the fibonacci sequence");

    stdin().read_line(&mut user_input).expect("Error reading the user input");

    let user_input: u8 = user_input.trim().parse().expect("Error parsing the number to u8");
    
    print_fibonacci_sequence(user_input);
}

fn print_fibonacci_sequence(num:u8) {
    let mut last:u32 = 1;
    let mut next:u32 = 1;
    let mut index:u8 = 0;

    while index < num {
        let temp = next;
        next = last+next; //This can overflow but idk how to handle exceptions correctly yet :(
        last = temp;
        println!("{}",last);
        index += 1; 
    }
}
