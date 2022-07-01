use std::io;

fn main() {
    let mut input_word = String::new();

    println!("input your word : ");
    io::stdin().read_line(&mut input_word);

    for str_slice in input_word.split_whitespace() {
        match str_slice.chars().nth(0).unwrap() {
            'a' | 'e' | 'i' | 'o' | 'u' => print!("{} ", format!("{}-hay", str_slice.trim())),
            _ => print!("{} ", format!("{}-{}ay", &str_slice[str_slice.chars().next().unwrap().len_utf8()..].trim(), str_slice.chars().nth(0).unwrap())),
        };
    }
}