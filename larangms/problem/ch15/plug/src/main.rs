use std::io;

fn main() {
    let count = read_int();
    let mut sum: i32 = 0;

    for _n in 0..count {
        sum += read_int();
    }

    println!("{}", sum - (count - 1));
}

fn read_int() -> i32 {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Fail to  read line");

    let result = buffer.trim().parse::<i32>().unwrap();
    result    
}