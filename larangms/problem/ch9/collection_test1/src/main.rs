use std::{io, collections::HashMap};

fn main() {
    let mut input = String::new();
    let mut input_vec: Vec<usize> = Vec::new();
    let mut scores: HashMap<usize, usize> = HashMap::new();
    let mut sum: usize = 0;

    io::stdin().read_line(&mut input).expect("Fail to read line");

    for val in input.split_whitespace() {
        let digit = val.trim().parse::<usize>().expect("please type a number");
        let count = scores.entry(digit).or_insert(0);
        *count += 1;

        input_vec.push(digit);
        sum += digit;
    }

    input_vec.sort();

    let input_len =  input_vec.len();    
    let mode = scores.iter().max_by_key(|entry | entry.1).unwrap().0;
    let average: f32 = (sum / input_len) as f32;
    let mut median: f32 = 0.0;
    let half  = input_len / 2; 

    if input_len % 2 == 0 {    
        median = ((input_vec[half - 1] + input_vec[half]) / 2) as f32;
    }
    else {
        median = input_vec[half] as f32;   
    }

    println!("average :{}", average);
    println!("median :{}", median);
    println!("mode :{}", mode);
    
}
