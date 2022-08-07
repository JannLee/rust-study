use std::io;

fn main() {
    let cases = get_positive_integer();

    for _case in 0..cases {
        println!("{}", get_car_price());
    }
}

fn get_positive_integer() -> u64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to get integer");
    input.trim().parse().unwrap()
}

fn get_car_price() -> u64 {
    let mut price = get_positive_integer();
    let options = get_positive_integer();

    for _opt in 0..options {
        let mut option_info = String::new();
        io::stdin().read_line(&mut option_info).expect("Failed to read the option information");
        let option_info: Vec<&str> = option_info.split_whitespace().collect();
        price += option_info[0].parse::<u64>().unwrap() * option_info[1].parse::<u64>().unwrap();
    }
    
    price
}