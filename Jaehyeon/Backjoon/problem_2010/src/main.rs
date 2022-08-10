use std::io;

fn main() {
    let mut total_plug_count = 1;

    let multi_plug_count = get_i32();
    for _ in 0..multi_plug_count {
        total_plug_count -= 1;

        let plug_count = get_i32();
        total_plug_count += plug_count;
    }
    println!("{}", total_plug_count);
}

fn get_i32() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().parse().unwrap();
}