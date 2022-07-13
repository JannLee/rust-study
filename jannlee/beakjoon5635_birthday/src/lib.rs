use std::io;

pub fn read_number() -> i32 {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse() {
        Ok(count) => count,
        Err(_) => panic!("Error case_count")
    }
}

pub fn read_student() -> (String, i32, i32, i32) {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let items: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
 
    (
        items[0].clone(),
        match items[1].parse() {
            Ok(number) => number,
            Err(_) => panic!("parse error")
        },
        match items[2].parse() {
            Ok(number) => number,
            Err(_) => panic!("parse error")
        },
        match items[3].parse() {
            Ok(number) => number,
            Err(_) => panic!("parse error")
        }
    )
}

pub fn solution(students : Vec<(String, i32, i32, i32)>) -> (String, String)
{
    let mut youngest = (String::new(), 0, 0, 0);
    let mut oldest = (String::new(), 99, 99, 9999);

    for student in students {
        if youngest.3 < student.3 ||
        (youngest.3 == student.3 && youngest.2 < student.2) ||
        (youngest.3 == student.3 && youngest.2 == student.2 && youngest.1 < student.1) {
            youngest = student.clone();
        }

        if oldest.3 > student.3 ||
        (oldest.3 == student.3 && oldest.2 > student.2) ||
        (oldest.3 == student.3 && oldest.2 == student.2 && oldest.1 > student.1) {
            oldest = student;
        }
    }

    (youngest.0, oldest.0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution(vec!
            [(String::from("Mickey"), 1, 10, 1991),
            (String::from("Alice"), 30, 12, 1990),
            (String::from("Tom"), 15, 8, 1993),
            (String::from("Jerry"), 18, 9, 1990),
            (String::from("Garfield"), 20, 9, 1990)]), (String::from("Tom"), String::from("Jerry")));
    }
}