extern crate beakjoon5635_birthday;

fn main() {
    let count = beakjoon5635_birthday::read_number();
    let mut i = 0;
    let mut students: Vec<(String, i32, i32, i32)> = Vec::new();

    while i < count {
        students.push(beakjoon5635_birthday::read_student());
        i = i + 1;
    }

    let s = beakjoon5635_birthday::solution(students);
    println!("{}", s.0);
    println!("{}", s.1);
}
