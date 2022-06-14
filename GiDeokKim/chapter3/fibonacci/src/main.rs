use std::io;

fn fibonacci(number: u64) -> u64 {
    match number {
        1 => 0,
        2 => 1,
        _ => fibonacci(number-2)+fibonacci(number-1)
    }
}

fn main() {
    loop {
        println!("Input number: ");

        let mut number = String::new();

        io::stdin().read_line(&mut number)
            .expect("Failed to read line");

        let number: u64 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("The Fahrenheit temperature is {}.", fibonacci(number));
        break;
    }
}