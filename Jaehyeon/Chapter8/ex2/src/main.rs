use std::io;

fn main() {
    let input = get_string();
    print!("{} -> ", input.trim());

    let result = to_pig_latin(&input);
    print!("{}", result);
}

fn get_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    return input;
}

fn to_pig_latin(str: &String) -> String{
    if is_consonant(str) {
        format!("{}-{}ay", str[1..].to_string().trim(), str.chars().nth(0).unwrap())
    }
    else {
        format!("{}-hay", str.trim())
    }
}

fn is_consonant(str: &String) -> bool {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let first = str.chars().nth(0).unwrap();
    
    for c in vowels {
        if first == c {
            return false;
        }
    }

    return true;
}