use std::io;

fn get_integer() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to get integer");
    input.trim().parse().unwrap()
}

fn main() {
    let cases = get_integer();
    let mut rookie: (i32, String);
    for _case in 0..cases {        
        let players = get_integer();
        rookie = (0, "".to_string());
        for _player in 0..players {
            let mut player_info = String::new();
            io::stdin().read_line(&mut player_info).expect("Failed to read player information");
            let player_info: Vec<&str> = player_info.split_whitespace().collect();

            let player = (player_info[0].parse().unwrap(), player_info[1].to_string());
            if rookie.0 < player.0 {
                rookie = player;
            }
        }
        println!("{}", rookie.1);
    }
}