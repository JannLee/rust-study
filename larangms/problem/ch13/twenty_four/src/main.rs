use std::io;
use chrono::NaiveTime;

fn main() {
    let current = read_time();
    let start = read_time(); 
    let result = start - current;    
  
    let seconds = result.num_seconds() % 60;
    let minutes = (result.num_seconds() / 60) % 60;
    let hours = (result.num_seconds() / 60) / 60;
    
    println!("{:02}:{:02}:{:02}", hours, minutes, seconds);     
}

fn parse_string_to_vec(input: &str, sep: &str) -> Vec<String> {
    let result = input.trim().split(sep).map(str::to_string).collect();
    result
}

fn read_time() -> NaiveTime {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Fail to  read line");

    let input = parse_string_to_vec(&buffer, ":");
    
    let result = NaiveTime::from_hms(input[0].parse().unwrap(),input[1].parse().unwrap(),input[2].parse().unwrap());
    result
}