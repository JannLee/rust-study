extern crate rand;

use rand::Rng;
use std::collections::HashMap;

fn main() {
    const NUM_COUNT: u32 = 10;

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
    println!("Average: {}, Median: {}, Mode: {}", average, get_mean(&numbers), get_mode(&numbers));
}

fn get_mean(numbers: &Vec<u32>) -> f64 {
    if numbers.len() % 2 == 0 { // 짝수면 가운데에 있는 데이터들의 평균 
        let mut total = *numbers.get(numbers.len() / 2).unwrap();
        total += *numbers.get(numbers.len() / 2 - 1).unwrap();
        
        total as f64 / 2.0
    }
    else {
        *numbers.get(numbers.len() / 2).unwrap() as f64
    }
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