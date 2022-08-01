fn main() {
    let tc = read_int();
    for _ in 0..tc {
        let s = read_int();
        let n = read_int();

        let mut result = s;
        for _ in 0..n {
            let (q, p) = read_pair();
            result += q * p;
        }
        println!("{}", result);
    }
}

fn read_str() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn read_int() -> i32 {
    read_str().parse().unwrap()
}

fn read_pair() -> (i32, i32) {
    let vec: Vec<i32> = read_str()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (vec[0], vec[1])
}
