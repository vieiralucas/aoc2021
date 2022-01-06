use std::{error::Error, fmt::Display, str::FromStr};

#[derive(Debug, Default)]
struct Pattern {
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    e: bool,
    f: bool,
    g: bool,
}

impl FromStr for Pattern {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            a: s.contains('a'),
            b: s.contains('b'),
            c: s.contains('c'),
            d: s.contains('d'),
            e: s.contains('e'),
            f: s.contains('f'),
            g: s.contains('g'),
        })
    }
}

#[derive(Debug)]
struct VecToSmallErr {
    needed: usize,
    provided: usize,
}

impl VecToSmallErr {
    fn new(needed: usize, provided: usize) -> Self {
        Self { needed, provided }
    }
}

impl Display for VecToSmallErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VecToSmallErr, needed: {}, but got only: {}",
            self.needed, self.provided
        )
    }
}

impl Error for VecToSmallErr {}

#[derive(Debug)]
struct Input {
    patterns: [Pattern; 10],
    output: [Pattern; 4],
}

impl FromStr for Input {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: [&str; 2] = s
            .split(" | ")
            .collect::<Vec<&str>>()
            .try_into()
            .map_err(|e: Vec<_>| Box::new(VecToSmallErr::new(2, e.len())))?;

        let mut patterns: Vec<Pattern> = vec![];
        for str in parts[0].split_whitespace() {
            match str.parse() {
                Ok(pattern) => patterns.push(pattern),
                Err(e) => return Err(e),
            }
        }

        let patterns: [Pattern; 10] = patterns
            .try_into()
            .map_err(|e: Vec<_>| Box::new(VecToSmallErr::new(10, e.len())))?;

        let mut output: Vec<Pattern> = vec![];
        for str in parts[1].split_whitespace() {
            match str.parse() {
                Ok(pattern) => output.push(pattern),
                Err(e) => return Err(e),
            }
        }

        let output: [Pattern; 4] = output
            .try_into()
            .map_err(|e: Vec<_>| Box::new(VecToSmallErr::new(4, e.len())))?;

        Ok(Self { patterns, output })
    }
}

fn part1() {
    let patterns = include_str!("../../inputs/day8/sample.txt")
        .trim()
        .lines()
        .map(|line| line.parse::<Input>().expect("line to be parsed as input"))
        .collect::<Vec<_>>();

    println!("Day 8, part 1: {:#?}", patterns);
}

fn part2() {
    let input = include_str!("../../inputs/day8/sample.txt");

    println!("Day 8, part 2: {}", 0);
}

fn main() {
    part1();
    part2();
}
