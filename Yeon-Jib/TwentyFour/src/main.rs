use std::io;
use std::fmt;

fn time_to_int(_time:&str) -> Result<u32,&'static str>{
    let mut time_details:Vec<u32> = Vec::new() ;
    let mut result = 0;
    let time:Vec<&str> = _time.split(":").collect();

    
    if time.len() < 3 {
        return Err("Too Small Unit");
    }
    for unit in time{
        time_details.push(unit.trim().parse().unwrap());
    }
    
    if time_details[0] > 24 || time_details[1] > 60 || time_details[2] > 60
    {
        return Err("Too Big Time");
    }
    else
    {
        result = (time_details[0]*60*60)+(time_details[1]*60)+time_details[2];
    }
    println!("{}",result);
    Ok(result)
}

fn check_dead_line(mut _now_time_info: u32, _begin_info:u32)-> String{
    let mut left_time = 0;

    if  _begin_info > _now_time_info{
        _now_time_info+=60*60*60;
    }
    left_time =_now_time_info- _begin_info;

    let hour = left_time/(60*60);

    left_time-=hour*(60*60);

    let min = left_time/(60);

    left_time-=min*60;

    let second = left_time;
    
    println!("{}:{}:{}", hour,min, second);
    format!("{}:{}:{}", hour,min, second).to_string()
}

fn main() {
    let mut input_time = String::new();
    let mut now_time_info:u32 = 0;
    let mut begin_info:u32 = 0;

    println!("please input mission begin time");

    io::stdin().read_line(&mut input_time)
        .expect("input error");
    match time_to_int(&input_time)
    {
        Ok(result) => now_time_info = result.clone(),
        Err(err) => println!("Error: {}", err),
    }
    let mut input_time = String::new();
    io::stdin().read_line(&mut input_time)
        .expect("input error");
    match time_to_int(&input_time)
    {
        Ok(result) => begin_info = result.clone(),
        Err(err) => println!("Error: {}", err),
    }
    println!("{}",check_dead_line(now_time_info, begin_info));
}
