use std::thread;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let is_quit = Arc::new(Mutex::new(false));

    let is_read_quit = Arc::clone(&is_quit);
    let read = thread::spawn(move || {
        loop {
            let f = &rx.try_recv();
            
            match f {
                Ok(str) => {println!("Got: {}", str);},
                Err(_) => {
                    if *is_read_quit.lock().unwrap() == true {
                        break;
                    }
                }
            };
        }
    });

    let mut writers = vec![];
    for i in 0..10 {
        let tx_clone = mpsc::Sender::clone(&tx);
        let writer = thread::spawn(move || {
            let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("thread"),
                    String::from(i.to_string())
                ];
    
            for val in vals {
                tx_clone.send(val).unwrap();
                thread::sleep(Duration::from_millis(50));
            }
        });
        writers.push(writer);
    }

    for writer in writers {
        writer.join().unwrap();    
    }

    {
        *is_quit.lock().unwrap() = true;
    }

    read.join().unwrap();
    println!("End");
}