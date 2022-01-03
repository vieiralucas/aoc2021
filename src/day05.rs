use std::collections::HashMap;
use std::ops::{Add, AddAssign};
use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let results: Vec<Result<isize, Self::Err>> =
            s.split(",").map(|s| s.parse::<isize>()).collect();
        let numbers: Result<Vec<isize>, Self::Err> = results.into_iter().collect();

        numbers.map(|ns| Point::new(ns[0], ns[1]))
    }
}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
    dir: Point,
}

fn get_dir(a: isize, b: isize) -> isize {
    if a == b {
        0
    } else if a > b {
        -1
    } else {
        1
    }
}

impl Line {
    fn new(p1: Point, p2: Point) -> Self {
        Self {
            p1,
            p2,
            dir: Point::new(get_dir(p1.x, p2.x), get_dir(p1.y, p2.y)),
        }
    }

    pub fn is_diagonal(&self) -> bool {
        self.p1.x != self.p2.x && self.p1.y != self.p2.y
    }
}

#[derive(Debug)]
struct LineIterator {
    dir: Point,
    next: Point,
    last: Point,
}

impl IntoIterator for &Line {
    type Item = Point;

    type IntoIter = LineIterator;

    fn into_iter(self) -> Self::IntoIter {
        LineIterator {
            dir: self.dir,
            next: self.p1,
            last: self.p2 + self.dir,
        }
    }
}

impl Iterator for LineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.next;

        if cur == self.last {
            None
        } else {
            self.next += self.dir;
            Some(cur)
        }
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let results: Vec<Result<Point, Self::Err>> =
            s.split(" -> ").map(|s| s.parse::<Point>()).collect();
        let numbers: Result<Vec<Point>, Self::Err> = results.into_iter().collect();

        numbers.map(|ns| Line::new(ns[0], ns[1]))
    }
}

pub fn part1() {
    let lines: Vec<Line> = include_str!("../inputs/day5/input.txt")
        .lines()
        .map(|line| line.parse().expect("Failed to parse line"))
        .collect();

    let mut board: HashMap<Point, isize> = HashMap::new();

    for line in lines.iter(){
        if line.is_diagonal() {
            continue;
        }

        for point in line.into_iter() {
            board.entry(point).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    println!(
        "Day 5, part 1: {}",
        board.values().filter(|v| **v > 1).count()
    );
}

pub fn part2() {
    let lines: Vec<Line> = include_str!("../inputs/day5/input.txt")
        .lines()
        .map(|line| line.parse().expect("Failed to parse line"))
        .collect();

    let mut board: HashMap<Point, isize> = HashMap::new();

    for line in lines.iter(){
        for point in line.into_iter() {
            board.entry(point).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    println!(
        "Day 5, part 2: {}",
        board.values().filter(|v| **v > 1).count()
    );
}
