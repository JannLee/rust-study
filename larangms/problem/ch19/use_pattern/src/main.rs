/*
    완전제곱수  문제 리팩토링 해보기 
    https://www.acmicpc.net/problem/1977
*/

use std::io;

fn main() {
    let first = read_i64();
    let last = read_i64();
    let mut value: Option<i64> = None;
    let mut sum: i64 = 0;

    for i in first ..= last  {
        if is_square(i) {
            match value {
                None => value = Some(i),
                _ => ()
            }
            sum += i;
        }                
    }

    match value {
        Some(val) => {   
            println!("{}", sum);
            println!("{}", val);
         },
        None => println!("{}", -1) 
    }

}

fn is_square(value: i64) -> bool {
   let tmp = ((value as f64).sqrt()) as i64; 
   if tmp * tmp == value {
    return true;
   }
   
   false
}

fn read_string() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Fail to  read line");
    buffer.trim().to_string()  
}

fn read_i64() -> i64 {   
    read_string().trim().parse::<i64>().unwrap()  
}