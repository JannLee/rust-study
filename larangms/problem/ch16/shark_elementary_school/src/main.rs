use std::{io, cmp::Ordering};

struct Student
{
    number: i32,
    friend: Vec<i32>,
    x: i32,
    y: i32,
}

struct SeatInfo
{
    friend: i32,
    empty: i32,
    y: i32,
    x: i32,
}

const DY:  [i32; 4] = [-1, 0, 1, 0];
const DX:  [i32; 4] = [0, -1, 0, 1];


fn main() {
    let n = read_usize();
    let mut students:Vec<Student> = Vec::new();
    let mut board = vec![vec![0; n]; n];
    
    for _ in 0..n * n {
        let input = read_int_vec();
        let student = Student {number: input[0], friend: input[1..].to_vec(), x: -1, y: -1};
        students.push(student);
    }  
    
    for i in 0..n * n {
        let mut info_list: Vec<SeatInfo> = Vec::new();

        for y in 0..n {
            for x in 0..n {
                if board[y][x] == 0 {
                    let (f, e) = get_friend_empty_cnt(&mut board, y as i32, x as i32, &students[i]);
                    
                    let info = SeatInfo {friend: f, empty: e, y: y as i32, x: x as i32};
                    info_list.push(info);  
                }
            }
        }

        info_list.sort_by(|a, b| {

            if a.friend > b.friend {
                return Ordering::Less;
            }
            if a.friend < b.friend {
                return Ordering::Greater;
            }

            if a.empty > b.empty {
                return Ordering::Less;
            }
            if a.empty < b.empty {
                return Ordering::Greater;
            }

            if a.y > b.y {
                return Ordering::Less;
            }
            if a.y < b.y {
                return Ordering::Greater;
            }

            if a.x > b.x {
                return Ordering::Less;
            }
            if a.x < b.x {
                return Ordering::Greater;
            }

            Ordering::Equal
        });


        let px = info_list[0].x;
        let py = info_list[0].y;

        students[i].y = py;
        students[i].x = px;
        board[py as usize][px as usize] = students[i].number;       
    }

    let mut result = 0;
    for i in 0..n * n {
        result += calc_satisfaction(&mut board, &mut students[i]);
    }
    
    println!("{}", result);
}


fn get_friend_empty_cnt(board: &mut Vec<Vec<i32>>, y: i32, x: i32, student: &Student) -> (i32, i32)
{
    let mut friend = 0;
    let mut empty = 0;

    for i in 0..4 {
        let ny = y + DY[i];
        let nx = x + DX[i];

        if ny < 0 || ny >= board[0].len() as i32 || nx < 0 || nx >= board[0].len() as i32 {
            continue;
        }

        if board[ny as usize][nx as usize] == 0 {
            empty += 1;
        }
        else {
            for f in 0..4 {
                if board[ny as usize][nx as usize] == student.friend[f] {
                    friend += 1;
                }
            }
        }
    }

    (friend, empty)
}


fn calc_satisfaction(board: &mut Vec<Vec<i32>>, student: &Student) -> i32 {
    let mut point = 0;
    let y = student.y;
    let x = student.x;

    for i in 0..4 {
        let ny = y + DY[i];
        let nx = x + DX[i];

        if ny < 0 || ny >= board[0].len() as i32 || nx < 0 || nx >= board[0].len() as i32 {
            continue;
        }

        for f in 0..4 {
            if board[ny as usize][nx as usize] == student.friend[f] {
                if point == 0 {
                    point += 1;
                }
                else {
                    point *= 10;
                }
            }
        }
    }

    point
}


//////////////////////////////
/// input func
//////////////////////////////
fn read_string() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Fail to  read line");
    buffer.trim().to_string()  
}

fn read_usize() -> usize {   
    read_string().trim().parse::<usize>().unwrap()  
}

fn read_int_vec() -> Vec<i32> {   
    read_string().trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()) .collect()
}