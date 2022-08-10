use std::io;

fn main() {
    let mut result: Vec<i32> = Vec::new();
    let count = read_int();

    for _n in 0..count {       
        let mut total_price = read_int();
        let opt_cnt = read_int();

        for _i in 0..opt_cnt {
            let opt = read_int_tuple();
            total_price += (opt.0 * opt.1);
        }

        result.push(total_price);       
    }

    for val in result {
        println!("{}", val);
    }
}


fn read_int() -> i32 {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Fail to  read line");

    let result = buffer.trim().parse::<i32>().unwrap();
    result    
}

fn read_int_tuple() -> (i32, i32) {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Fail to  read line");
    let input_vec: Vec<&str> = buffer.trim().split_whitespace().collect();

    let result = (input_vec[0].parse::<i32>().unwrap(), input_vec[1].parse::<i32>().unwrap());
    result    
}