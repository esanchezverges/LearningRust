use std::io;

fn main() -> ! {
    println!("Conversion of fahrenheit to celsious");
    loop {
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Error reading user input");

        let fahrenheit:f64 = match user_input
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        let celsius = get_celsius_from_fahrenheit(fahrenheit);

        println!("{} Fahrenheit == {} Celsius",fahrenheit,celsius);
    }
}

fn get_celsius_from_fahrenheit(fahrenheit: f64 ) -> f64 {
    (fahrenheit - 32.0)/1.8
}
