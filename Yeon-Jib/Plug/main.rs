use std::io;

fn input_u32() -> u32{
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    guess
}

fn main() {
    let mut multitab_number = 0;
    let mut consents_number: Vec<u32> = Vec::new();

    println!("input multitab number");
    multitab_number = input_u32();

    for index in 0..multitab_number
    {
        consents_number.push(input_u32());
    }
    let result:u32 = consents_number.iter().sum();
    println!("result:{}",result-multitab_number+21);

}
