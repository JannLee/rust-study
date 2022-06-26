use std::io;

pub mod jayee {
    pub enum EqualType {
        Equal(i32),
        Less(i32)
    }
}

fn main() {
    let m = read_num();
    let n = read_num();
    
    let min = match get_square(m) {
        jayee::EqualType::Equal(i) => i,
        jayee::EqualType::Less(i) => i
    };
    let max = match get_square(n) {
        jayee::EqualType::Equal(i) => i,
        jayee::EqualType::Less(i) => (i - 1)
    };

    let mut total = 0;
    for square in min..=max {
        total += square * square;
    }
    
    println!("{}", total);
    if max < min {
        println!("-1");
    }
    else {
        println!("{}", min * min);
    }
}

fn read_num() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().unwrap()
}

fn get_square(num : i32) -> jayee::EqualType {
    for i in 0..=100 {
        if (i * i) == num {
            return jayee::EqualType::Equal(i);
        }
        else if num < (i * i) {
            return jayee::EqualType::Less(i);
        }
    }

    return jayee::EqualType::Equal(100);
}