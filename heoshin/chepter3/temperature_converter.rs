use std::io;

enum Temperature {
    C(f64),
    F(f64)
}

fn unit_convert(temperature: Temperature) -> Temperature {
    match temperature {
        Temperature::C(c) => Temperature::F(c * 1.8 + 32.),
        Temperature::F(f) => Temperature::C(f * 1.8 + 32.)
    }
}

fn main() {
    println!("Please Enter to number and unit: ");

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)
        .expect("Failed to read line");
    
    let user_input_count = user_input.trim().chars().count();
    println!("user_input_count: {user_input_count}");
    
    let temperature: f64 = match user_input.trim()[0..user_input_count-1].trim().parse() {
        Ok(num) => num,
        Err(_) => return
    };
    println!("{}", temperature);
    
    let temperature = match user_input.as_bytes()[user_input_count-1] as char {
        'C' => unit_convert(Temperature::C(temperature)),
        'F' => unit_convert(Temperature::F(temperature)),
        _ => return
    };

    match temperature {
        Temperature::C(c) => println!("{}C", c),
        Temperature::F(f) => println!("{}F", f),
    }
}