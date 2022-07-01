mod task1;
mod task2;
use std::io;
use std::collections::HashMap;

// list <부서명>
// list
// Add <사람> to <부서>

fn main() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new(); 
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let input: Vec<&str> = input.split_whitespace().collect();
        if input.len() == 0 {
            continue;
        }

        if input[0] == "list" {
            if input.len() > 1 {
                let dept = input[1].to_string();
                if map.contains_key(&dept) {
                    for name in map.entry(dept).or_default().iter() {
                        println!("{}", name);
                    }
                }
            } else {
                for dept in map.iter() {
                    println!("{}", dept.0);
                    for name in dept.1 {
                        println!("    {}", name);
                    }
                }
            }
        } else if input.len() > 3 && input[0] == "Add" && input[2] == "to" {
            let name = input[1];
            let dept = input[3];
            let vec = map.entry(dept.to_string()).or_insert(Vec::new());
            vec.push(name.to_string());
            vec.sort()
        }
    }
}
