fn main() {
    let n = read_int();
    let mut sum = 1;
    for _ in 0..n {
        sum += read_int() - 1;
    }
    println!("{}", sum);
}

fn read_int() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
