use std::{io, result};

fn main() {
    println!("변환할 온도를 입력하세요");
    println!("ex) 20 C or 20 F");

    let mut input = String::new();
    io::stdin().read_line(&mut input);

    let iter = input.split_whitespace();
    let vec = iter.collect::<Vec<&str>>();

    let temperature : f64  = vec[0].parse::<f64>().unwrap();
    let temperature_type = vec[1].to_uppercase();

    if temperature_type == "F" {
        let result = fahrenheit_to_celsius(temperature);
        println!("{:.2} °C", result);      
    }
    else  {
        let result = celsius_to_fahrenheit(temperature);  
        println!("{:.2} °F", result);      
    }

}

fn celsius_to_fahrenheit(temperature : f64) -> f64
{    
    temperature * 1.8 + 32.0
}

fn fahrenheit_to_celsius(temperature : f64) -> f64
{   
    (temperature - 32.0) / 1.8   
}