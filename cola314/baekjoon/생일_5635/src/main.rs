use std::io;

fn main() {
    let n = read_int();
    let mut students = Vec::new();
    for _ in 0..n {
        let input = read_string();
        let student = parse_student(&input);
        students.push(student)
    }
    let result = solve(&mut students);
    println!("{}\n{}", result.0.name, result.1.name);
}

fn solve<'a>(students: &'a mut Vec<Student>) -> (&'a Student, &'a Student) {
    students.sort();
    let oldest = students.last().unwrap();
    let youngest = students.first().unwrap();
    (oldest, youngest)
}

fn parse_student(input: &str) -> Student {
    let vec: Vec<&str> = input.split_whitespace().collect();
    Student {
        name: vec[0].to_string(),
        birthday: Date {
            day: vec[1].parse().unwrap(),
            month: vec[2].parse().unwrap(),
            year: vec[3].parse().unwrap(),
        },
    }
}

fn read_int() -> i32 {
    read_string().parse().unwrap()
}

fn read_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Student {
    birthday: Date,
    name: String,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Date {
    year: i32,
    month: i32,
    day: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_student_test() {
        let expected = Student {
            name: "Mickey".to_string(),
            birthday: Date {
                day: 1,
                month: 10,
                year: 1991,
            },
        };
        let actual = parse_student("Mickey 1 10 1991");
        assert_eq!(expected, actual);
    }

    #[test]
    fn example1_test() {
        let mut students = vec![
            parse_student("Mickey 1 10 1991"),
            parse_student("Alice 30 12 1990"),
            parse_student("Tom 15 8 1993"),
            parse_student("Jerry 18 9 1990"),
            parse_student("Garfield 20 9 1990"),
        ];
        let result = solve(&mut students);
        assert_eq!(result.0.name, "Tom", "Youngest student is not correct");
        assert_eq!(result.1.name, "Jerry", "Oldest student is not correct");
    }
}
