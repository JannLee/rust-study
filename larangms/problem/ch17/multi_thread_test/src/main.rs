use std::f64;
use std::time::SystemTime;
use std::thread::{self, JoinHandle};


fn prime_count(start: i32, end: i32) -> i32 {
    let mut result: i32 = 0;

    for i in start..end {
        if is_prime(i) {
            result += 1;
        }
    }

    result
}

fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }

    let sq:i32 = (f64::sqrt(n as f64)) as i32;

    for i in 2..=sq {
        if n % i == 0 {
            return false;
        }
    }
    
    true
}


fn main() {   

    let start: i32 = 1;
    let end: i32 = 2000001;

    cal_single(start, end);
    cal_multi(start, end);
}

fn cal_single(start: i32, end: i32) {

    println!(" ========== call cal_single ============");    

    let t1 = SystemTime::now();
    
    let count = prime_count(start, end);

    let t2 = SystemTime::now();

    println!("prime count : {}", count);    
    println!("tick : {:?}", t2.duration_since(t1));
}

fn cal_multi(start: i32, end: i32) {

    println!(" ========== call cal_multi ============");    

    let t1 = SystemTime::now();    

    let mut count = 0;
    let mut threads: Vec<JoinHandle<i32>> = Vec::new();
    let gap = (end - 1) / 4;

    for i in 0..4 {
        let s =  start + i * gap;
        let e = s + gap;

        let handle = thread::spawn(move || {
            prime_count(s, e)
        });

        threads.push(handle);        
    }

    //count = threads[0].join().unwrap() + threads[1].join().unwrap();        

    /*
    while let Some(t) = threads.pop() {
        count += t.join().unwrap();        
    }
    */
    
    for t in threads {
        count += t.join().unwrap();        
    }    


    let t2 = SystemTime::now();

    println!("prime count : {}", count);    
    println!("tick : {:?}", t2.duration_since(t1));
}
