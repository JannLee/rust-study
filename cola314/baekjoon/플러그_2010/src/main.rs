use std::io::stdin;

fn main() {
    if let Some(n) = read_int() {
        let mut sum = 1;
        for _ in 0..n {
            if let Some(cur) = read_int() {
                sum += cur - 1;
            }
        }
        println!("{}", sum);
    }
}

fn read_int() -> Option<i32> {
    let mut input = String::new();
    let result = stdin().read_line(&mut input);
    if let Ok(_) = result {
        return input.trim().parse().ok();
    }
    None
}
