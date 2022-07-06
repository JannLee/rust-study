use std::{io, collections::HashMap};

fn main() {
    let mut result: Vec<String> = Vec::new();
    let case_cnt = read_number();

    for _i in 0..case_cnt {       
        let player_cnt =read_number();
        let mut infomation: HashMap<u128, String> = HashMap::new();

        for _i in 0..player_cnt {
            let player = read_player();
            infomation.insert(player.0, player.1);
        }

        result.push(infomation.iter().max_by_key(|entry| entry.0).unwrap().1.to_string());
    }

    for val in result {
        println!("{}", val)
    }   
}

fn read_player() -> (u128, String) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fail to  read line");

    let tmp: Vec<&str> = input.trim().split_whitespace().collect();
    (tmp[0].parse::<u128>().unwrap(), tmp[1].to_string())
}    

fn read_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fail to  read line");
    
    if let Ok(number) = input.trim().parse::<i32>() {
        number
    } else {
        0
    }
}
