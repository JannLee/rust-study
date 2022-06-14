use std::io;

enum Temperature {
    Celcius(f64),
    Fahrenheit(f64),
}

fn unit_convertor(temperature: Temperature) -> f64 {
    match temperature {
        Temperature::Celcius(c) => c*1.8+32.0,
        Temperature::Fahrenheit(f) => (f-32.0)*5.0/9.0,
    }
}

fn main() {
    loop {
        println!("Input Celcius temperature: ");

        let mut number = String::new();

        io::stdin().read_line(&mut number)
            .expect("Failed to read line");

        let number: f64 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let temperature = Temperature::Celcius(number);

        println!("The Fahrenheit temperature is {}.", unit_convertor(temperature));
        break;
    }

    loop {
        println!("Input Fahrenheit temperature: ");

        let mut number = String::new();

        io::stdin().read_line(&mut number)
            .expect("Failed to read line");

        let number: f64 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let temperature = Temperature::Fahrenheit(number);

        println!("The Celcius temperature is {}.", unit_convertor(temperature));
        break;
    }
}
