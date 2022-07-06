use std::io;

fn main() {
    let test_case = get_i32();
    for _i in 0..test_case {        
        let football_player_count = get_i32();
        let mut most_expensive_player: (i32, String) = (0, "".to_string());
        for _j in 0..football_player_count {
            
            let player = get_football_player();
            if most_expensive_player.0 < player.0 {
                most_expensive_player = player;
            }
        }
        println!("{}", most_expensive_player.1);
    }
}

fn get_i32() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().parse().unwrap();
}

fn get_football_player() -> (i32, String) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let player: Vec<&str> = input.split_whitespace().collect();
    (player[0].parse().unwrap(), player[1].to_string())
}