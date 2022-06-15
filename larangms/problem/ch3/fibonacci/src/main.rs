use std::{io, result};

fn main() {
    println!("생성할 피보나치 수열 값(N 번째)을 입력하세요: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input);

    let N : i32 = input.trim().parse().unwrap();

    println!("N 번째 피보나치 수열 {}", fibonacci(N));
}

fn fibonacci(value : i32) -> i32 {
    if value < 2 {
        return value;
    }

    fibonacci(value - 1) + fibonacci(value - 2)
}
