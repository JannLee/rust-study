use std::io;
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let grid_n = get_usize_integer();
    let student_infos = get_student_infos(grid_n);
    let mut class_room : Vec<Vec<usize>> = vec![vec![0; grid_n]; grid_n];
    let mut best_pos : Position;

    for student_info in student_infos.iter() {
        best_pos = get_best_position(grid_n, student_info, &mut class_room);
        class_room[(-best_pos.x) as usize][(-best_pos.y) as usize] = student_info.student_number;
    }
    
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    let size = student_infos.len();

    for _index in 0..size {
        let mut class = class_room.clone();
        let student_info = student_infos[_index].clone();
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += get_score(grid_n, &student_info, &mut class);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", *counter.lock().unwrap());
}

fn get_usize_integer() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to get integer");
    input.trim().parse().unwrap()
}

#[derive(Clone)]
struct Student {
    student_number : usize,
    favorite_friends : [usize; 4]
}

fn get_student_info() -> Student {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read the student information");
    let information: Vec<usize> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    
    Student {
        student_number: information[0],
        favorite_friends: [information[1], information[2], information[3], information[4]],
    }
} 

fn get_student_infos(grid_n : usize) -> Vec<Student> {
    let num_students = grid_n * grid_n;
    let mut result = Vec::new();
    let mut student_info : Student;
    
    for _ in 0..num_students {
        student_info = get_student_info();
        result.push(student_info);
    }

    result
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
struct Position {
    favorite_friends : usize,
    empty_seats : usize,
    x : i32,
    y : i32,
}

fn get_best_position(grid_n : usize, student_info : &Student, class_room : &mut Vec<Vec<usize>>) -> Position {
    let mut positions = Vec::new();
    let dx : [i32; 4] = [0, 0, 1, -1];
    let dy : [i32; 4] = [1, -1, 0, 0];

    for _x in 0..grid_n {
        for _y in 0..grid_n {
            if class_room[_x][_y] != 0 {
                continue;
            }

            let mut _empty_seats : usize = 0;
            let mut _favorite_friends : usize = 0;
            for _index in 0..4 {
                let coordinate_x = _x as i32 + dx[_index];
                let coordinate_y = _y as i32 + dy[_index];
                if coordinate_x < 0 || coordinate_y < 0 || 
                    coordinate_x >= grid_n as i32 || coordinate_y >= grid_n as i32 {
                    continue;
                }

                if class_room[coordinate_x as usize][coordinate_y as usize] == 0 {
                    _empty_seats += 1;
                }
                else {
                    for _friend_index in 0..4 {
                        if class_room[coordinate_x as usize][coordinate_y as usize] == student_info.favorite_friends[_friend_index] {
                            _favorite_friends += 1;
                            break;
                        }
                    }
                }
            }
            positions.push(Position { x: -(_x as i32), y: -(_y as i32), empty_seats: _empty_seats, favorite_friends: _favorite_friends});
        }
    }
    positions.sort();

    positions.pop().clone().unwrap()
}

fn get_score(grid_n : usize, student_info : &Student, class_room : &mut Vec<Vec<usize>>) -> i64 {
    let mut score = 0;
    let dx : [i32; 4] = [0, 0, 1, -1];
    let dy : [i32; 4] = [1, -1, 0, 0];

    for _x in 0..grid_n {
        for _y in 0..grid_n {
            if class_room[_x][_y] != student_info.student_number {
                continue;
            }

            for _index in 0..4 {
                let coordinate_x = _x as i32 + dx[_index];
                let coordinate_y = _y as i32 + dy[_index];
                if coordinate_x < 0 || coordinate_y < 0 || 
                    coordinate_x >= grid_n as i32 || coordinate_y >= grid_n as i32 {
                    continue;
                }

                for _favorite_index in 0..4 {
                    if class_room[coordinate_x as usize][coordinate_y as usize] == student_info.favorite_friends[_favorite_index] {
                        match score {
                            0 => score = 1,
                            _ => score *= 10,
                        };
                    }
                }
            }
        }
    }

    score
}