use std::io;

fn main() {
    let testcase = get_i32();

    for _ in 0..testcase {
        let mut car_price = get_i32();
        let option_count = get_i32();
        
        for _ in 0..option_count {
            let option: (i32, i32) = get_option();

            car_price += option.0 * option.1;
        }
        println!("{}", car_price);
    }

}

fn get_i32() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().parse().unwrap();
}

fn get_option() -> (i32, i32) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let option: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    return (option[0], option[1]);
}