extern crate rand;

use rand::Rng;
use std::collections::HashMap;

fn main() {
    const NUM_COUNT: u32 = 100;

    let mut rng = rand::thread_rng();
    let mut numbers: Vec<u32> = Vec::new();
    
    for _i in 1..=NUM_COUNT {
        numbers.push(rng.gen_range(0, NUM_COUNT));
    }

    let mut total = 0;
    for i in &numbers {
        print!("{} ", i);
        total += i;
    }

    numbers.sort();
    println!("\n");

    for i in &numbers {
        print!("{} ", i);
    }
    println!("\n");

    let average = (total as f32) / (numbers.len() as f32);
    println!("Average: {}, Median: {}, Mode: {}", average, &numbers[((NUM_COUNT / 2) - 1) as usize], get_mode(&numbers));
}

fn get_mode(numbers: &Vec<u32>) -> u32 {
    let mut number_map: HashMap<u32, u32> = HashMap::new();

    let mut mode = 0;
    let mut mode_count = 0;

    for i in numbers {
        let count = number_map.entry(*i).or_insert(0);
        *count += 1;
    }
    
    for (key, value) in &number_map {
        if mode_count < *value {
            mode = *key;
            mode_count = *value;
        }
    }
    
    return mode;
}