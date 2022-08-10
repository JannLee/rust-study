use std::io;

fn main() {
    let cases = get_positive_integer();
    
    println!("{}", get_num_plug(cases));
    
}

fn get_positive_integer() -> i64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to get integer");
    input.trim().parse().unwrap()
}

fn get_num_plug(cases : i64) -> i64 {
    let mut result = 1 - cases;
    for _case in 0..cases {
        result += get_positive_integer();
    }
    
    result
}