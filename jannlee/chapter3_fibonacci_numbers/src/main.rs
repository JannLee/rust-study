use std::io;

fn main() {
    println!("Please input number!");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line.");

    let number: i32 = input.trim().parse().unwrap();
    let mut index = 0;
    let mut answer = 0;
    let mut fn0 = 0;
    let mut fn1 = 1;
    while index <= number {
        if index == 0 {
            answer = 0;
        }
        else if index == 1 {
            answer = 1;
        }
        else {
            answer = fn0 + fn1;
            fn0 = fn1;
            fn1 = answer;
        }
        index = index + 1;
    }

    println!("{}", answer);

}
