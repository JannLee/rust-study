use std::io;
use std::collections::HashMap;

fn main() {
    
    let mut company : HashMap<String, Vec<String>> = HashMap::new();
    loop
    {
        let command = get_string();
        let c = command.chars().nth(0).unwrap();

        if c == 'a' || c == 'A'
        {
            let commands: Vec<&str> = command.split_whitespace().collect();
            add(&mut company, commands[3], commands[1]);
        }
        else if c == 'q' || c == 'Q'
        {
            break;
        }
        else if c == 'p' || c == 'P'
        {
            print(&company);
        }
    }
    
}

fn get_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    return input;
}

fn add(company: &mut HashMap<String, Vec<String>>, department: &str, employee: &str) {
    let employees = match company.get_mut(department)
    {
        Some(i) => i,
        None => { 
            company.insert(department.to_string(), Vec::new());
            company.get_mut(department).unwrap()
        }
    };
    employees.push(employee.to_string());
    employees.sort();
}

fn print(company: &HashMap<String, Vec<String>>) {
    for (department, employees) in company {
        print!("{}: ", department);
        
        for employee in employees {
            print!("{}, ", employee);
        }
        println!();
    }
}