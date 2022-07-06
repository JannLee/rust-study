use std::io;

fn main() {
    let mut buffer = String::new();
    let mut digit = vec![0, 0];

    for i in 0..digit.len() {
        buffer.clear();
        io::stdin().read_line(&mut buffer).expect("fail to read line");
        digit[i] = buffer.trim().parse::<i64>().expect("please type a number");        
    }

    let mut value: i64 = -1;
    let mut sum: i64 = 0;

    for i in digit[0]..digit[1] + 1  {
        if is_square(i) {
            if value == -1 {
                value = i;
            }    

            sum += i;
        }                
    }

    if value == -1 {
        println!("{}", value);
    }
    else {
        println!("{}", sum);
        println!("{}", value);
    }
    
}

fn is_square(value: i64) -> bool {
   let tmp = ((value as f64).sqrt()) as i64;

   if tmp * tmp == value {
    return true;
   }

   false
}