use std::collections::HashMap;
use std::io;

fn main() {
    let mut emp_list = HashMap::new();

    loop {
        println!("The 1st word is command:");
        println!("ex1 Add Sally to Engineering");
        println!("ex2 Show Engineering member list");
        println!("ex2 Quit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error");

        let command_words :Vec<&str> = input.split_ascii_whitespace().collect();
        let name =command_words[1];
        let depart = command_words[3];

        match command_words[0] {
            "Add" => {
                emp_list.insert(name, depart);
            }
            "Show" => {
                    let mut emp_vec: Vec<String> = Vec::new();
                    for (k, v) in &emp_list {                            
                        emp_vec.push(format!("{k} at {v}"));
                    }
                    
                    emp_vec.sort();

                    println!("{:?}", emp_vec);    
                }
            "Quit" => break,
            _ => (),
        }
    }
}