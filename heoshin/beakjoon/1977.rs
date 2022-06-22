use std::io;
use std::collections::HashSet;

fn read_number() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn get_perfect_squeares() -> HashSet<u32> {
    let mut perfect_squares = HashSet::<u32>::new();
    for i in 1..=100 {
        perfect_squares.insert(i * i);
    }
    perfect_squares
}

fn main() {
    let m: u32 = read_number();
    let n: u32 = read_number();

    let perfect_squares = get_perfect_squeares();

    let mut sum: u32 = 0;
    let mut first_square: Option<u32> = None;
    
    for i in m..=n {
        if let Some(n) = perfect_squares.get(&i) {
            first_square.get_or_insert(i);
            sum += n;
        }
    }

    if let Some(n) = first_square {
        println!("{}\n{}", sum, n);
    } else {
        println!("-1");
    }
}