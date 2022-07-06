use std::io;

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

    let player: Vec<&str> = input.split_whitespace().collect();

    (player[0].parse().unwrap(), player[1].to_string())
}

fn main() {
    let n: i32 = read_number();
    
    for i in 0..n {
        let p = read_number();

        let mut best = (0, String::new());
        for j in 0..p {
            let player = read_player();
            if player.0 > best.0 {
                best = player;
            }
        }
        println!("{}", best.1);
    }
}