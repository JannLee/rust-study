use std::{io, collections::HashMap};

fn main() {
    let n = read_int();
    for _ in 0..n {
        let p = read_int();
        let mut map = HashMap::new();
        for _ in 0..p {    
            let (price, name) = read_member();
            map.insert(price, name);
        }
        let result = map.iter().max_by_key(|x| x.0);
        if let Some(result) = result {
            println!("{}", result.1);
        }
    }
}

fn read_int() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

fn read_member() -> (i32, String) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let vec: Vec<&str> = input.split_whitespace().collect();
    (vec[0].parse().unwrap(), vec[1].to_string())
}