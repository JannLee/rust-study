use std::io;

fn main() {
    let mut celsius = String::new();
    println!("Input celsius: ");
    io::stdin().read_line(&mut celsius).expect("Failed to read line");

    let celsius: u32 = match celsius.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    println!("{}", to_celsius(celsius));
}

fn to_celsius(celsius :u32) -> f64 {
    (f64::try_from(celsius).unwrap() * 9.0 / 5.0) + 32.0
}
