use std::collections::{HashMap, HashSet};
use std::vec::Vec;

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Default, Debug, Clone)]
struct Board {
    value_to_pos: HashMap<u64, Pos>,
    pos_to_value: HashMap<Pos, u64>,
    marked: HashSet<u64>,
    win_value: Option<u64>,
}

impl Board {
    pub fn insert(&mut self, value: u64, pos: Pos) {
        self.value_to_pos.insert(value, pos);
        self.pos_to_value.insert(pos, value);
    }

    pub fn mark(&mut self, value: u64) {
        if self.value_to_pos.contains_key(&value) {
            self.marked.insert(value);
        }

        if self.winner() {
            self.win_value = Some(value);
        }
    }

    pub fn winner(&self) -> bool {
        if self.win_value.is_some() {
            return true;
        }

        // verticals
        for i in 0..5 {
            let mut win = true;
            for j in 0..5 {
                if !self.marked.contains(&self.pos_to_value[&Pos::new(i, j)]) {
                    win = false;
                    break;
                }
            }

            if win {
                return true;
            }
        }

        // horizontals
        for i in 0..5 {
            let mut win = true;
            for j in 0..5 {
                if !self.marked.contains(&self.pos_to_value[&Pos::new(j, i)]) {
                    win = false;
                    break;
                }
            }

            if win {
                return true;
            }
        }

        false
    }

    pub fn score(&self) -> u64 {
        self.pos_to_value
            .values()
            .filter(|v| !self.marked.contains(v))
            .sum()
    }
}

fn parse_data(s: &str) -> (Vec<Board>, Vec<u64>) {
    let mut lines = s.lines();

    let numbers: Vec<u64> = lines
        .next()
        .expect("Failed to parse numbers line")
        .split(',')
        .map(|s| s.parse().expect("Failed to parse u64"))
        .collect();

    let _ = lines.next().expect("empty line after numbers");

    let mut boards: Vec<Board> = vec![];

    let mut i = 0;
    let mut board: Board = Board::default();

    for line in lines {
        if i > 0 && i % 5 == 0 {
            boards.push(board.clone());
            board = Board::default();
            i = 0;
            continue;
        }

        for (j, raw) in line.split_whitespace().enumerate() {
            let value = raw.trim().parse().expect("Failed to parse u64");
            board.insert(value, Pos::new(i, j));
        }
        i += 1;
    }
    boards.push(board);

    (boards, numbers)
}

pub fn part1() {
    let (mut boards, numbers) = parse_data(include_str!("../inputs/day4/input.txt"));

    let mut final_score = 0;
    'outer: for n in numbers {
        for board in &mut boards {
            board.mark(n);
            if board.winner() {
                final_score = board.score() * n;
                break 'outer;
            }
        }
    }

    println!("Day 4, part 1: {}", final_score);
}

pub fn part2() {
    let (mut boards, numbers) = parse_data(include_str!("../inputs/day4/input.txt"));

    let mut last_winner = 0;
    for n in numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            if board.winner() {
                continue;
            }

            board.mark(n);
            if board.winner() {
                last_winner = i;
            }
        }
    }

    println!(
        "Day 4, part 2: {}",
        boards[last_winner].score() * boards[last_winner].win_value.unwrap()
    );
}
