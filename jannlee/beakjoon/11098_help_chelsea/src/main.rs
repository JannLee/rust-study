use std::io;

fn main() {
    let case_count: i32 = read_number();
    let mut i = 0;

    while i < case_count {
        let player_count = read_number();
        let mut j = 0;
        let mut best = (0, String::new());
        while j < player_count {
            let player = read_player();
            if player.0 > best.0 {
                best = player;
            }
            j = j + 1;
        }
        println!("{}", best.1);
        i = i + 1;
    }
}

fn read_number() -> i32 {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse() {
        Ok(count) => count,
        Err(_) => panic!("Error case_count")
    }
}

fn read_player() -> (i32, String) {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let items: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
 
    (
        match items[0].parse() {
            Ok(number) => number,
            Err(_) => panic!("parse error")
        },
        items[1].clone()
    )
}
