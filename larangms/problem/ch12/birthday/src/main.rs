use std::io;
use chrono::{NaiveDate, NaiveDateTime};

struct User {
    name: String,
    birthday: NaiveDate,
}

fn main() {
    
    let users = read_users();
    let (young_man, old_man) = solve(&users);

    println!("{}", young_man.name);
    println!("{}", old_man.name);    
}


fn solve(users : &[User]) -> (&User, &User) {   

    let young_man = users.iter().max_by_key(|u| u.birthday).unwrap();
    let old_man = users.iter().min_by_key(|u| u.birthday).unwrap();    

    (young_man, old_man)  
}


fn read_users() -> Vec<User> {
    
    let mut input_str = String::new();
    let mut result = Vec::new();

    let user_cnt = read_number();

    for _i  in 0..user_cnt {
        input_str.clear();        
        io::stdin().read_line(&mut input_str).expect("Fail to  read line");        

        let user = make_user(&input_str);
        result.push(user);
    }

    result
}


fn make_user(input_str: &str) -> User {

    let input = parse_string_to_vec(input_str);

    let name = input[0].clone();
    let birthday = NaiveDate::from_ymd(input[3].parse().unwrap(),
                                                input[2].parse().unwrap(),
                                                input[1].parse().unwrap());

    User { name, birthday }
}


fn read_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fail to  read line");
    
    if let Ok(number) = input.trim().parse::<i32>() {
        number
    } else {
        0
    }
}


fn parse_string_to_vec(input: &str) -> Vec<String> {
    let result = input.trim().split_whitespace().map(str::to_string).collect();
    result
}

/*
fn read_string() -> Vec<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fail to  read line");

    let result = input.trim().split_whitespace().map(str::to_string).collect();
    result
}
*/


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_test() {        
        let users = "\
Mickey 1 10 1991
Alice 30 12 1990
Tom 15 8 1993
Jerry 18 9 1990
Garfield 20 9 1990";

        let mut result = Vec::new();

        for line in users.lines() {
            let user = make_user(line);
            result.push(user);
        }

        let (young_man, old_man) = solve(&result);

        assert_eq!(young_man.name, "Tom", "젊은놈 아냐~~");
        assert_eq!(old_man.name, "Jerry", "늙은놈 아냐~~");
   
    }
}