use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
    marked: bool,
    value: u32,
}

#[derive(Debug)]
struct Board {
    values: HashMap<u32, Point>,
    rows: [u32; 5],
    columns: [u32; 5],
}

fn create_board(board_spec: &[&str]) -> Board {
    let mut board = Board {
        values: HashMap::new(),
        rows: [0, 0, 0, 0, 0],
        columns: [0, 0, 0, 0, 0],
    };
    for (x, row) in board_spec.iter().enumerate() {
        for (y, value) in row.split(" ").filter(|x| !x.is_empty()).enumerate() {
            let value = value.parse::<u32>().unwrap();
            let point = Point {
                x: x as u32,
                y: y as u32,
                marked: false,
                value,
            };
            board.values.insert(value, point);
        }
    }
    board
}

fn check_board(board: &mut Board, value: u32) -> i32 {
    if board.values.contains_key(&value) {
        let mut point = board.values.get_mut(&value).unwrap();
        point.marked = true;
        board.rows[point.x as usize] += 1;
        board.columns[point.y as usize] += 1;
        if board.rows[point.x as usize] == 5 || board.columns[point.y as usize] == 5 {
            return value as i32;
        }
    }
    -1
}

fn bingo(mut boards: Vec<Board>, values: Vec<u32>, first: bool) -> u32 {
    let mut boards_num = boards.len();
    let mut winners = HashSet::new();
    for value in values {
        for (i, board) in boards.iter_mut().enumerate() {
            if winners.contains(&i) {
                continue;
            }
            let win = check_board(board, value);
            if win > -1 {
                winners.insert(i);
                if first {
                    return win as u32
                        * board
                            .values
                            .values()
                            .filter(|x| !x.marked)
                            .fold(0, |acc, x| acc + x.value);
                }
                boards_num -= 1;
                if boards_num == 0 {
                    return win as u32
                        * board
                            .values
                            .values()
                            .filter(|x| !x.marked)
                            .fold(0, |acc, x| acc + x.value);
                }
            }
        }
    }
    0
}

fn part1(input: String) -> String {
    let lines: Vec<&str> = input.split("\n").filter(|x| !x.is_empty()).collect();
    let values = lines[0]
        .split(",")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::new();
    for board in lines[1..].chunks(5) {
        boards.push(create_board(board));
    }
    bingo(boards, values, true).to_string()
}

fn part2(input: String) -> String {
    let lines: Vec<&str> = input.split("\n").filter(|x| !x.is_empty()).collect();
    let values = lines[0]
        .split(",")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::new();
    for board in lines[1..].chunks(5) {
        boards.push(create_board(board));
    }
    bingo(boards, values, false).to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
