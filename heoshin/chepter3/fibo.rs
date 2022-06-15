use std::io;

fn fibo(number: u64) -> u64 {
    match number {
        1..=2 => number,
        _ => fibo(number - 2) + fibo(number - 1)
    }
}

fn main() {
    println!("Input number: ");

    let mut number = String::new();

    io::stdin().read_line(&mut number)
        .expect("Failed to read line");

    let number: u64 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    println!("{}", fibo(number));
}
