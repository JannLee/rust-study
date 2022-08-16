use std::io;

fn main() {
    let grid_n = get_positive_integer();
    let student_infos = set_student_infos(grid_n);
    let mut class_room : Vec<Vec<i64>> = vec![vec![0; grid_n]; grid_n];
    let mut best_positions : Vec<Option<Position>> = Vec::new();
    let mut result = 0;

    for student_info in student_infos.iter() {
        let mut all_positions = get_best_position(grid_n, student_info, &mut class_room);
        let best_pos = all_positions.pop();
        let set_pos = best_pos.clone().unwrap();
        best_positions.push(best_pos);
        class_room[(-set_pos.x) as usize][(-set_pos.y) as usize] = student_info.student_number;
    }
    
    for student_info in student_infos.iter() {
        result += get_score(grid_n, student_info, &mut class_room);
    }

    println!("{}", result);
}

fn get_positive_integer() -> usize {
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

fn set_student_infos(grid_n : usize) -> Vec<Student> {
    let num_students = grid_n * grid_n;
    let mut result = Vec::new();
    let mut student_info : Student;
    
    for _ in 0..num_students {
        student_info = set_student_info();
        result.push(student_info);
    }

    result
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Position {
    favorite_friends : i64,
    empty_seats : i64,
    x : i64,
    y : i64,
}

fn get_best_position(grid_n : usize, student_info : &Student, class_room : &mut Vec<Vec<i64>>) -> Vec<Position> {
    let mut positions = Vec::new();
    let dx : [i64; 4] = [0, 0, 1, -1];
    let dy : [i64; 4] = [1, -1, 0, 0];

    for _x in 0..grid_n {
        for _y in 0..grid_n {
            if class_room[_x][_y] != 0 {
                continue;
            }

            let mut _empty_seats : i64 = 0;
            let mut _favorite_friends : i64 = 0;
            for _index in 0..4 {
                let coordinate_x = _x as i64 + dx[_index];
                let coordinate_y = _y as i64 + dy[_index];
                if coordinate_x < 0 || coordinate_y < 0 || 
                    coordinate_x >= grid_n as i64 || coordinate_y >= grid_n as i64 {
                    continue;
                }
                if class_room[coordinate_x as usize][coordinate_y as usize] == 0 {
                    _empty_seats += 1;
                }
                else {
                    for _friend_index in 0..4 {
                        let favorite_student = student_info.favorite_friends[_friend_index];
                        if class_room[coordinate_x as usize][coordinate_y as usize] == favorite_student {
                            _favorite_friends += 1;
                            break;
                        }
                    }
                }
            }
            positions.push(Position { x: -(_x as i64), y: -(_y as i64), empty_seats: _empty_seats, favorite_friends: _favorite_friends});
        }
    }
    positions.sort();

    positions
}

fn get_score(grid_n : usize, student_info : &Student, class_room : &mut Vec<Vec<i64>>) -> i64 {
    let mut score = 0;
    let dx : [i64; 4] = [0, 0, 1, -1];
    let dy : [i64; 4] = [1, -1, 0, 0];

    for _x in 0..grid_n {
        for _y in 0..grid_n {
            if class_room[_x][_y] != student_info.student_number {
                continue;
            }

            for _index in 0..4 {
                let coordinate_x = _x as i64 + dx[_index];
                let coordinate_y = _y as i64 + dy[_index];
                if coordinate_x < 0 || coordinate_y < 0 || 
                    coordinate_x >= grid_n as i64 || coordinate_y >= grid_n as i64 {
                    continue;
                }

                for _favorite_index in 0..4 {
                    if class_room[coordinate_x as usize][coordinate_y as usize] == student_info.favorite_friends[_favorite_index] {
                        if score == 0 {
                            score = 1;
                        }
                        else {
                            score *= 10;
                        }
                    }
                }
            }
        }
    }

    score
}