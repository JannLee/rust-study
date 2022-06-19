use std::io;

fn main() {
    loop {
        println!("Please input temperature with unit(F or C).");
        println!("- input example 1: 10F");
        println!("- input example 2: 20.3C");
    
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line.");
    
        let temperature = input.trim().to_string().to_lowercase();
        let unit = &temperature[temperature.len() - 1..temperature.len()];
        let number = &temperature[..temperature.len() - 1].parse::<f32>().unwrap();

        match unit {
            "c" => {
                println!("{} => {}F", temperature, number * 9.0 / 5.0 + 32.0);
            },
            "f" => {
                println!("{} => {}C", temperature, (number - 32.0) * 5.0 / 9.0);
            },
            _ => continue
        };
    }
}
