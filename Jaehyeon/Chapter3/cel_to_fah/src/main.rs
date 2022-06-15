use std::io;

fn main() {
    let mut input = String::new();
    println!("Input temperature: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    println!("Input to celsius: {}", to_celsius(input));
    println!("Input to fahrenheit: {}", to_fahrenheit(input));
}

fn to_fahrenheit(celsius :f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn to_celsius(fahrenheit :f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}