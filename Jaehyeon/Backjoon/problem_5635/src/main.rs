use std::io;

struct Student {
    name: String,
    day: i32,
    month: i32,
    year: i32,
}

impl Student {
    fn clone(&mut self, other: &Student) {
        self.name = other.name.clone();
        self.year = other.year;
        self.month = other.month;
        self.day = other.day;
    }

    fn is_older_than(&self, other: &Student) -> bool {
        if self.year < other.year {
            return true;
        }
        else if self.year == other.year {
            if self.month < other.month {
                return true;
            }
            else if self.month == other.month {
                if self.day < other.day {
                    return true;
                }
            }
        }
        return false;
    }

    fn is_younger_than(&self, other: &Student) -> bool {
        if self.year > other.year {
            return true;
        }
        else if self.year == other.year {
            if self.month > other.month {
                return true;
            }
            else if self.month == other.month {
                if self.day > other.day {
                    return true;
                }
            }
        }
        return false;
    }
}

fn main() {
    let student_count = get_i32();
    
    let mut youngest = Student {name: "".to_string(), year: 0, month: 0, day: 0};
    let mut oldest = Student {name: "".to_string(), year: 0, month: 0, day: 0};

    for _ in 0..student_count {
        let student = get_student();

        if youngest.year == 0 || youngest.is_older_than(&student) {
            youngest.clone(&student);
        }
        
        if oldest.year == 0 || oldest.is_younger_than(&student) {
            oldest.clone(&student);
        }
    }

    println!("{}\n{}", youngest.name, oldest.name);
}

fn get_i32() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().parse().unwrap();
}

fn get_student() -> Student {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let student: Vec<&str> = input.split_whitespace().collect();
    return Student {
        name: student[0].to_string(),
        day: student[1].parse().unwrap(),
        month: student[2].parse().unwrap(),
        year: student[3].parse().unwrap(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn younger_test() {
        let a = Student {name: "a".to_string(), year: 1993, month: 9, day: 8};
        let younger_b = Student {name: "b".to_string(), year: 1994, month: 9, day: 8};
        let younger_c = Student {name: "c".to_string(), year: 1993, month: 10, day: 8};
        let younger_d = Student {name: "d".to_string(), year: 1993, month: 9, day: 9};

        let older_b = Student {name: "b".to_string(), year: 1992, month: 9, day: 8};
        let older_c = Student {name: "c".to_string(), year: 1993, month: 8, day: 8};
        let older_d = Student {name: "d".to_string(), year: 1993, month: 9, day: 7};

        assert!(a.is_younger_than(&older_b));  // true 기대
        assert!(a.is_younger_than(&older_c));  // true 기대
        assert!(a.is_younger_than(&older_d));  // true 기대

        assert!(!a.is_younger_than(&younger_b));  // false 기대 (!로 true로 변경)
        assert!(!a.is_younger_than(&younger_c));  // false 기대 (!로 true로 변경)
        assert!(!a.is_younger_than(&younger_d));  // false 기대 (!로 true로 변경)
    }

    #[test]
    fn older_test() {
        let a = Student {name: "a".to_string(), year: 1993, month: 9, day: 8};
        let younger_b = Student {name: "b".to_string(), year: 1994, month: 9, day: 8};
        let younger_c = Student {name: "c".to_string(), year: 1993, month: 10, day: 8};
        let younger_d = Student {name: "d".to_string(), year: 1993, month: 9, day: 9};

        let older_b = Student {name: "b".to_string(), year: 1992, month: 9, day: 8};
        let older_c = Student {name: "c".to_string(), year: 1993, month: 8, day: 8};
        let older_d = Student {name: "d".to_string(), year: 1993, month: 9, day: 7};

        assert!(a.is_older_than(&younger_b));  // true 기대
        assert!(a.is_older_than(&younger_c));  // true 기대
        assert!(a.is_older_than(&younger_d));  // true 기대

        assert!(!a.is_older_than(&older_b));  // false 기대 (!로 true로 변경)
        assert!(!a.is_older_than(&older_c));  // false 기대 (!로 true로 변경)
        assert!(!a.is_older_than(&older_d));  // false 기대 (!로 true로 변경)
    }
}