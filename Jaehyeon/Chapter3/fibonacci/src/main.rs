use std::io;

fn main() {
    let mut fibonaccis: Vec<u64> = Vec::new();

    let mut count = String::new();
    println!("Input fibonacci count: ");
    io::stdin().read_line(&mut count).expect("Failed to read line");

    let count: u32 = match count.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let count: usize = usize::try_from(count).unwrap();
    fibonacci(count, &mut fibonaccis);
    
    for n in 1..(fibonaccis.len()) {
        println!("{} ", fibonaccis[n]);
    };
}

fn fibonacci(count: usize, fibonaccis: &mut Vec<u64>) {
    if count < 2 {
        fibonaccis.resize(2, 0);
    }
    else {
        fibonaccis.resize(count + 1, 0);
    }

    fibonaccis[0] = 0;
    fibonaccis[1] = 1;

    for n in 2..=(count) {
        fibonaccis[n] = fibonaccis[n - 1] + fibonaccis[n - 2];
    };
}