use std::io;

fn main() {
    let m = get_positive_integer();
    let n = get_positive_integer();
    
    let (result, min_square) = get_square_number(m, n);

    match result {
        0 => println!("-1"),
        _ => println!("{}\n{}", result, min_square),
    }
}

fn get_positive_integer() -> u64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to get integer");
    input.trim().parse().unwrap()
}

fn get_square_number(m: u64, n: u64) -> (u64, u64) {
    let (mut result, mut min_square) = (0, 0);
    let mut square;

    for number in 1..=100 {
        square = number * number;

        if square < m {
			continue;
        }
        else if square > n {
			break;
        }

		if min_square == 0 {
            min_square = square;
        }

		result += square;
    }

    (result, min_square)
}