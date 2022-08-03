use std::io;

fn main() {
    let current_time : i32 = get_seconds();
    let started_time : i32 = get_seconds();
    let mut seconds = started_time - current_time;
    if seconds < 0 {
        seconds = seconds + 24 * 3600;
    }

    print_time(seconds);
}

fn get_seconds() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to get integer");
    let seperated_times: Vec<&str> = input.split(":").collect();

    let mut seconds : i32 = 0;
    let mut unit : i32 = 3600;
    for seperated_time in seperated_times
    {
        let mut converted : i32 = seperated_time.to_string().trim().parse().unwrap();
        converted = converted  * unit;
        unit = unit / 60;
        seconds = seconds + converted;
    }

    seconds
}

fn print_time(seconds : i32) {

    if seconds >= 86400 {
        panic!("seconds must be less than 86400, got {}.",
        seconds);
    }

    let hours : i32 = seconds / 3600;
    let minutes = seconds / 60 % 60;
    let seconds = seconds % 60;
    let time = format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds);
    
    print!("{}", time);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic(expected = "seconds must be less than 86400")]
    fn test_print_time() {
        print_time(86400);
    }
}