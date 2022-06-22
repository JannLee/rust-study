use std::io::stdin;

fn main() {
    let min = get_i32();
    let max = get_i32();

    // psn(perfect sqaure number)
    let mut min_psn = 0;
    let mut sum_psn = 0;

    let mut number = 1;
    while number * number <= 10_000 {
        let psn = number * number;
        if psn >= min {
            if psn <= max {
                if min_psn == 0 {
                    min_psn = psn;
                }
                sum_psn = sum_psn + psn;
            }
            else {
                break;
            }    
        }

        number = number + 1;
    }

    if sum_psn != 0 {
        println!("{}", sum_psn);
        println!("{}", min_psn);
    }
    else {
        println!("-1");
    }
}

fn get_i32() -> i32 {
    let mut input = String::new();

    stdin().read_line(& mut input).expect("Failed to read input line");

    return match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error number format");
            -1
        }
    };
}
