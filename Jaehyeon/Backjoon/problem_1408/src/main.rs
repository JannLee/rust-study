use std::io;

fn main() {
    let current = get_time_string();
    let start = get_time_string();
    
    let current_sec = time_string_to_i32(&current);
    let start_sec = time_string_to_i32(&start);
    
    let mut duration: i32 = start_sec - current_sec;
    if duration < 0 {
        duration += 24 * 3600;
    }

    println!("{}", time_i32_to_string(duration));
}

fn get_time_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input;
}

fn time_string_to_i32(time: &String) -> i32 {
    let times: Vec<&str> = time.trim().split(":").collect();

    let hours: i32 = times[0].parse().unwrap();
    let minutes: i32 = times[1].parse().unwrap();
    let mut seconds: i32 = times[2].parse().unwrap();
    
    seconds += hours * 3600;
    seconds += minutes * 60;

    return seconds;
}

fn time_i32_to_string(time: i32) -> String {
    let mut remain = time;
    let hours = remain / 3600;
    remain = remain % 3600;

    let minutes = remain / 60;
    remain = remain % 60;

    let seconds = remain;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}