use std::io;

fn main() {
    let mut m = String::new();
    let mut n = String::new();

    io::stdin().read_line(&mut m)
        .expect("Failed to read line");

    let m: u64 = match m.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    io::stdin().read_line(&mut n)
        .expect("Failed to read line");

    let n: u64 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    
    let mut result = 0;
    let mut min_square = 0;

    for number in 1..=100 {
        if number * number < m {
			continue;
        }

		if number * number > n {
			break;
        }

		if min_square==0 {
            min_square = number * number;
        }

		result += number * number;
    }
    
    match result {
        0 => println!("-1"),
        _ => {
            println!("{}", result);
            println!("{}", min_square);
        }
    }
}