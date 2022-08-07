use std::io::stdin;

fn main() {
    let max_case = get_numbers()[0];
    let mut i = 0;

    while i < max_case {
        let mut sum = get_numbers()[0];
        let max_option = get_numbers()[0];

        let mut j = 0;
        while j < max_option {
            let option = get_numbers();
            sum = sum + option[0] * option[1];
            j = j + 1;
        }

        println!("{}", sum);
        i = i + 1;
    }
}

fn get_numbers() -> Vec<i32> {
    let mut input = String::new();

    stdin().read_line(& mut input).unwrap();

    input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
