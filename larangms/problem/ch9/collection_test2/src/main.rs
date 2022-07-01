use std::{io};


fn main() {

    println!("Please enter an English word");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fail to  read line");
    
    let input_vec: Vec<char> = input.trim().chars().collect();
    let mut output_vec: Vec<char> = Vec::new();

    let consonants = ['B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q',
    'T', 'R', 'S', 'V', 'W', 'X', 'Z'];

    let first = input_vec[0].to_ascii_uppercase();

    if first.is_alphabetic() {
        if consonants.contains(&first) {
            output_vec.append(&mut input_vec[1..].to_vec());
            output_vec.append(&mut format!("-{}ay", input_vec[0]).chars().collect());
            /*
            let mut t: Vec<char> = format!("-{}ay", input_vec[0]).chars().collect();
            output_vec.append(&mut t);
            */        
        }
        else  {        
            output_vec.append(&mut input_vec[0..].to_vec());
            output_vec.append(&mut format!("-{}ay", 'h').chars().collect());
            /*
            let mut t: Vec<char> = format!("-{}ay", 'h').chars().collect();
            output_vec.append(&mut t);
            */        
        }
    }
    else {
        output_vec.append(&mut input_vec[0..].to_vec());
    }   

    let out = String::from_iter(output_vec);
    println!("{}", out);

}
