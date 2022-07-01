use std::{io, collections::HashMap};
use unicase::UniCase;

fn main() {
    let mut input = String::new();
    let mut department: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("input your commad");
        println!("ex) A : add user, P : print informations, Q : Exit");

        input.clear();
        io::stdin().read_line(&mut input).expect("Fail to read line");
        let command = input.trim().to_uppercase().chars().next();

        match command {
            Some('A') => {
                add_user(&mut department);
            },
            Some('P') => {
                print_info(&mut department);
            },
            Some('Q') => {
                break;
            },
           _ => (),
        }
    }
    
}


fn add_user(department: &mut HashMap<String, Vec<String>>) {
    println!("input user");
    println!("ex) department_name : user_name");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fail to read line");

    let info: Vec<&str> = input.trim().split(':').collect();
    let users = department.entry(info[0].to_string()).or_insert(Vec::new());
    
    if !users.contains(&info[1].to_string()) {
        users.push(info[1].to_string());
    }
}

fn print_info(department: &mut HashMap<String, Vec<String>>) {
    println!("enter the department name");
    println!("ex) ALL or department name");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fail to read line");
    
    let all = UniCase::new("all");
    let d_name = UniCase::new(input.trim());   

    if all == d_name  {
        println!("{:#?}", department);
    }
    else  { 
        if department.contains_key(&d_name.to_string()) {
            println!("{:#?}", department[&d_name.to_string()]);        
        }  
        else  {            
            println!("No department information.")
        }        
    }
}
