use std::io;

fn main() {
    let testcase = get_number();

    for _ in 0..testcase {
        let mut car_price = get_number();
        let n = get_number();
        
        for _ in 0..n {
            let (q, p) = get_option();

            car_price += q * p;
        }
        println!("{}", car_price);
    }
}

fn get_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().unwrap()
}

fn get_option() -> (i32, i32) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let option: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    (option[0], option[1])
}