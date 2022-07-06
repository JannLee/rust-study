use std::io;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    let mut numbers: Vec<i32> = input.split_whitespace()
        .map(|x|->i32{x.parse().unwrap()})
        .collect();

    let length = numbers.len();
    if length > 0 {
        numbers.sort();

        let mut sum = 0;
        let mut map = HashMap::new();
        let mut mode = (0, 0);
        for number in numbers.iter() {
            sum = sum + number;
            let item = map.entry(number).or_insert(0);
            *item += 1;
            if mode.1 < *item {
                mode = (*number, *item);
            }
        }
        
        println!("mean: {}", sum / length as i32);

        if length % 2 == 0 { // 짝수
            println!("median: {}", (numbers[length / 2 - 1] + numbers[length / 2]) / 2);
        }
        else {
            println!("median: {}", numbers[length / 2]);
        }

        println!("mode: {}", mode.0);
    }
}
