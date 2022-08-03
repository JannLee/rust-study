use std::io;


struct Student{
    name:String,
    birthday:u32
}

fn get_student_info(&self, info_string:&str) {
    let infos: Vec<&str> = info_string.split_whitespace().collect();
    self.name = infos[0].parse::<u32>().unwrap();
    self.birth_year = infos[3];
    self.birth_month = infos[2];
    self.birth_date = infos[1];
 }


fn main() {
    let mut input = String::new();
    let mut student_info:Vec<Student> = Vec::new();
    let mut oldest_student = String::new();

    println!("please input student number: ");

    io::stdin().read_line(&mut input)
        .expect("Failed to student number");

    let student_number:i32 = input.trim().parse().unwrap();
    for _index in 0..student_number{
        let input_info = String::new();

        student_info.push(get_student_info(&input_info));

    }
    for _index in 0..student_number{
        if student_info._index.birthday > student_info._index+1.birthday{
            oldest_student = student_info._index.name
        }
        
    }
    
}
