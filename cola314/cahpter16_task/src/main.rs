use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let result = sum_pararrel(1, 1000000000);
    println!("{}", result);
}

fn sum(start: i128, end: i128) -> i128 {
    let mut result = 0;
    for i in start..end {
        result += i;
    }
    result
}

fn sum_pararrel(start: i128, end: i128) -> i128 {
    let chunck_num = 12;
    let chunck_size = (end - start) / chunck_num;

    let chuncks = (0..chunck_num).map(|i| {
        let s = start + i * chunck_size;
        let e = if i == chunck_num - 1 {
            s + (end - start) - chunck_size * (chunck_num - 1)
        } else {
            s + chunck_size
        };
        (s, e)
    });

    let acc = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for chunck in chuncks {
        let acc = Arc::clone(&acc);

        let handle = thread::spawn(move || {
            let cur = sum(chunck.0, chunck.1);
            *acc.lock().unwrap() += cur;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    return *acc.lock().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_test() {
        let result = sum(1, 1000000000);
        assert_eq!(result, 499999999500000000);
    }

    #[test]
    fn sum_pararrel_test() {
        let result = sum_pararrel(1, 1000000000);
        assert_eq!(result, 499999999500000000);
    }
}
