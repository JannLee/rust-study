struct Student {
    id: i32,
    friends: Vec<i32>,
}

struct Board {
    like: i32,
    empty: i32,
    y: i32,
    x: i32,
}

fn main() {
    let n = read_int();
    let mut students: Vec<Student> = Vec::new();
    for _ in 0..n * n {
        let student = read_student();
        students.push(student);
    }
    let mut board = vec![vec![0; n]; n];

    for student in &students {
        place(&mut board, student, n as i32);
    }

    let score = get_score(&board, &students, n as i32);
    print!("{}", score);
}

fn adjust_block(y: i32, x: i32, board_size: i32) -> Vec<(i32, i32)> {
    let blocks = vec![(y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)];
    blocks
        .into_iter()
        .filter(|x| 0 <= x.0 && x.0 < board_size && 0 <= x.1 && x.1 < board_size)
        .collect()
}

fn get_score(board: &Vec<Vec<i32>>, students: &Vec<Student>, board_size: i32) -> i32 {
    let mut result = 0;

    for y in 0..board_size as usize {
        for x in 0..board_size as usize {
            let student_id = board[y][x];
            let student = students.iter().find(|x| x.id == student_id).unwrap();
            let count = adjust_block(y as i32, x as i32, board_size)
                .iter()
                .map(|pos| board[pos.0 as usize][pos.1 as usize])
                .filter(|friend_id| student.friends.iter().any(|x| x == friend_id))
                .count();

            result += match count {
                1 => 1,
                2 => 10,
                3 => 100,
                4 => 1000,
                _ => 0,
            }
        }
    }
    result
}

fn place(board: &mut Vec<Vec<i32>>, student: &Student, board_size: i32) {
    let empty: Vec<(i32, i32)> = (0..board_size)
        .flat_map(|y| (0..board_size).map(move |x| (y, x)))
        .filter(|pos| board[pos.0 as usize][pos.1 as usize] == 0)
        .collect();

    let mut items: Vec<Board> = empty
        .iter()
        .map(|t| {
            let like = adjust_block(t.0, t.1, board_size)
                .iter()
                .map(|pos| board[pos.0 as usize][pos.1 as usize])
                .filter(|friend_id| student.friends.iter().any(|x| x == friend_id))
                .count();
            let empty_count = adjust_block(t.0, t.1, board_size)
                .iter()
                .filter(|pos| board[pos.0 as usize][pos.1 as usize] == 0)
                .count();
            Board {
                empty: empty_count as i32,
                like: like as i32,
                y: t.0,
                x: t.1,
            }
        })
        .collect();

    items.sort_by(|a, b| {
        b.like
            .cmp(&a.like)
            .then(b.empty.cmp(&a.empty))
            .then(a.y.cmp(&b.y))
            .then(a.x.cmp(&b.x))
    });

    let (y, x) = (items[0].y, items[0].x);
    board[y as usize][x as usize] = student.id;
}

fn read_int() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn read_student() -> Student {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let vec: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    Student {
        id: vec[0],
        friends: vec[1..].to_vec(),
    }
}
