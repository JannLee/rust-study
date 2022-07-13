use std::io;

fn read_number() -> i32 {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse().unwrap()
}

fn read_person_info() -> (String, u32) {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let inputs: Vec<&str> = input.split_whitespace().collect();
    
    let birthday: u32 = format!("{:0>4}{:0>2}{:0>2}", inputs[3], inputs[2], inputs[1]).parse().unwrap();

    (inputs[0].to_string(), birthday)
}

fn main() {
    let n = read_number();

    let mut oldest_person_name = String::new();
    let mut oldest_birthday = u32::MAX;
    let mut newest_person_name = String::new();
    let mut newest_birthday = u32::MIN;

    for _i in 0..n {
        let person_info = read_person_info();
        
        if oldest_birthday > person_info.1 {
            oldest_person_name = person_info.0.clone();
            oldest_birthday = person_info.1;
        }
        if newest_birthday < person_info.1 {
            newest_person_name = person_info.0.clone();
            newest_birthday = person_info.1;
        }
    }
    
    println!("{newest_person_name}\n{oldest_person_name}");
}