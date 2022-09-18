use std::io;
use std::collections::HashMap;

#[derive(Debug)]
struct Student
{
    id: i32,
    friends: Vec<i32>
}

impl Clone for Student {
    fn clone(&self) -> Student {
        Student{id:self.id, friends:self.friends.clone()}
    }
}

#[derive(Debug)]
struct Seat
{
    x: i32,
    y: i32
}
impl Copy for Seat { }
impl Clone for Seat {
    fn clone(&self) -> Seat {
        Seat{x:self.x, y:self.y}
    }
}

fn main() {
    let n = get_i32();
    let mut students: Vec<Student> = Vec::new();

    for _ in 0..n * n {
        let student = get_student();
        students.push(student.clone());
    }

    let mut classroom = vec![vec![0; n]; n];
    for student in &students {
        let mut seats = get_seats_in_first_case(&classroom, &student.friends);
        
        if 1 < seats.len() {
            seats = get_seats_in_second_case(&classroom, &seats, n as i32);

            if 1 < seats.len() {
                let seat = get_seats_in_third_case(&seats, n as i32);

                seats.clear();
                seats.push(seat);
            }
        }
        classroom[seats[0].y as usize][seats[0].x as usize] = student.id;
    }

    // Make score
    calc_score(&classroom);
}

// 자리 잡기
// 1. 빈칸 중 좋아하는 학생이 인접한 칸에 가장 많은 칸
// 2. 1을 만족하는 칸이 여러 개, 인접한 칸 중 빈칸이 가장 많은 자리 
// 3. 2를 만족하는 칸이 여러 개, 행 번호가 낮은 곳으로, 그런곳도 여러 개라면 열 번호가 낮은 곳으로 

// 1번 조건 만족 
fn get_seats_in_first_case(classroom: &Vec<Vec<i32>>, friends: &Vec<i32>) -> Vec<Seat> {
    let mut seats: Vec<Seat> = Vec::new();
    let mut max_adjacent_count = 0;

    for y in 0..classroom.len() {
        for x in 0..classroom[y].len() {
            let adjacent_count = get_adjacent_friends_count(&classroom, Seat{y: y as i32, x: x as i32}, &friends, classroom.len() as i32);
            if adjacent_count == max_adjacent_count {
                seats.push(Seat{y: y as i32, x: x as i32});
            }
            else if max_adjacent_count < adjacent_count {
                seats.clear();
                max_adjacent_count = adjacent_count;
                seats.push(Seat{y: y as i32, x: x as i32});
            }
        }
    }
    return seats;
}

fn get_adjacent_friends_count(classroom: &Vec<Vec<i32>>, seat: Seat, friends: &Vec<i32>, n: i32) -> i32 {
    let mut adjacent_count = 0;

    for adjacent in get_adjacent_seats(&seat, n) {
        for friend in friends {
            if classroom[adjacent.y as usize][adjacent.x as usize] == *friend {
                adjacent_count += 1;
            }
        }
    }
    return adjacent_count;
}

// 2번 조건 만족
fn get_seats_in_second_case(classroom: &Vec<Vec<i32>>, seats: &Vec<Seat>, n: i32) -> Vec<Seat> {
    let mut result: Vec<Seat> = Vec::new();
    let mut max_empty_count = 0;
    
    for seat in seats {
        let mut empty_count = 0;

        for adjacent in get_adjacent_seats(&seat, n) {
            if classroom[adjacent.y as usize][adjacent.x as usize] == 0 {
                empty_count += 1;
            }

            if max_empty_count < empty_count {
                max_empty_count = empty_count;
                result.clear();
                result.push(seat.clone());
            }
            else if max_empty_count == empty_count {
                result.push(seat.clone());
            }
        }
    }

    return result;
}

// 3번 조건 만족
fn get_seats_in_third_case(seats: &Vec<Seat>, n: i32) -> Seat {
    let mut min_seats: Vec<Seat> = Vec::new();
    let mut min_row = n + 1;
    for seat in seats {
        if min_row < seat.y {
            min_seats.clear();
            min_row = seat.y;
            min_seats.push(*seat);
        }
        else if min_row == seat.y {
            min_seats.push(*seat);
        }
    }

    if 1 < min_seats.len() {
        for seat in seats {
            if min_row < seat.x {
                min_seats.clear();
                min_row = seat.x;
                min_seats.push(seat.clone());
            }
            else if min_row == seat.x {
                min_seats.push(seat.clone());
            }
        }
    }
    return min_seats[0];
}

fn calc_score(classroom: &Vec<Vec<i32>>) -> i32 {

    return score;
}

fn get_adjacent_seats(seat: &Seat, n: i32) -> Vec<Seat> {
    let temp: Vec<Seat> = vec![Seat{y: seat.y + 1, x: seat.x}, Seat{y: seat.y - 1, x: seat.x}, Seat{y: seat.y, x: seat.x + 1}, Seat{y: seat.y, x: seat.x - 1}];
    temp.into_iter().filter(|current| 0 <= current.y && current.y < n && 0 <= current.x && current.x < n).collect()
}

fn get_i32() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().parse().unwrap();
}

fn get_student() -> Student {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let vec: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    Student { id: vec[0], friends: vec[1..].to_vec() }
}