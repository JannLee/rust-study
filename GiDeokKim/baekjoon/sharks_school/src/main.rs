use std::io;

fn main() {
    let grid_n = get_positive_integer();
    let student_infos = set_student_infos(grid_n);
    
    for student_info in student_infos.iter() {    
        println!("{:?}", student_info);
    }
    
}

fn get_positive_integer() -> u64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to get integer");
    input.trim().parse().unwrap()
}

#[derive(Debug)]
struct Student {
    student_number : i64,
    favorite_friends : [i64; 4]
}

fn set_student_info() -> Student {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read the student information");

    let information: Vec<i64> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let result = Student {
        student_number: information[0],
        favorite_friends: [information[1], information[2], information[3], information[4]],
    };

    result
} 

fn set_student_infos(grid_n : u64) -> Vec<Student> {
    let num_students = grid_n * grid_n;
    let mut result = Vec::new();
    let mut student_info : Student;
    
    for _ in 0..num_students {
        student_info = set_student_info();
        result.push(student_info);
    }

    result
}