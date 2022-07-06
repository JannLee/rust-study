use std::io;

fn solve(m: i32, n: i32) -> Option<Result> {
    let candidates: Vec<i32> = (1..10000)
        .map(|x| x * x)
        .filter(|p| m <= *p && *p <= n)
        .collect();
    if candidates.is_empty() {
        None
    } else {
        Some(Result {
            sum_val: candidates.iter().sum(),
            min_val: *candidates.iter().min().unwrap(),
        })
    }
}

fn main() {
    let m = read_int();
    let n = read_int();
    let result = solve(m, n);
    match result {
        Some(result) => println!("{}\n{}", result.sum_val, result.min_val),
        None => println!("{}", -1),
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
struct Result {
    sum_val: i32,
    min_val: i32,
}

fn read_int() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let result = solve(60, 100);
        let expected = Option::Some(Result {
            sum_val: 245,
            min_val: 64,
        });
        assert_eq!(result, expected);
    }

    #[test]
    fn example2() {
        let result = solve(75, 80);
        let expected = Option::None;
        assert_eq!(result, expected);
    }
}