use std::collections::HashMap;

fn average(vec_list: &Vec<i32>) {
    let mut total_sum: f32 = 0.0;

    let vec_len = vec_list.len() as f32;

    for entry in vec_list {
        total_sum += *entry as f32;
    }
    println!("Average = {}", total_sum / vec_len);
}

fn median(vec_list: &mut Vec<i32>) {
    vec_list.sort();
    let center = vec_list.len() / 2;
    if vec_list.len() % 2 == 1 {
        println!("Median = {}", vec_list[center]);
    }
    else {
        println!("Median = {} and {}", vec_list[center], vec_list[center - 1]);
    }
}

fn mode(vec_list: &Vec<i32>) {
    let mut vec_mode = HashMap::new();
    for vec_entry in vec_list {
        let count = vec_mode.entry(vec_entry).or_insert(0);
        *count += 1;
    }
    
    let mut most_value: i32 = 0;
    let mut most_key: i32 = 0;
    for (key, value) in &vec_mode {
        if most_value < *value {
            most_key = *(*key);
            most_value = *value;
        }
    }
    println!("Mode = {}", most_key);
}

fn main() {
    let mut vec_list: Vec<i32> = Vec::new();
    vec_list.push(10);
    vec_list.push(100);
    vec_list.push(30);
    vec_list.push(75);
    vec_list.push(13);
    vec_list.push(13);

    average(&vec_list);
    median(&mut vec_list);
    mode(&vec_list);
}