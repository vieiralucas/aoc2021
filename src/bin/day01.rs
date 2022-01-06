use std::fs;

fn part1() {
    let input: Vec<u64> = fs::read_to_string("inputs/day1/input.txt")
        .expect("Failed to read input")
        .lines()
        .map(|l| l.parse().expect("Failed to convert line to u64"))
        .collect();

    let mut result = 0;
    for (i, curr) in input.iter().enumerate() {
        if i < 1 {
            continue;
        }

        if let Some(prev) = input.get(i - 1) {
            if prev < curr {
                result += 1;
            }
        }
    }

    println!("Day 1, part 1: {}", result);
}

fn part2() {
    let input: Vec<u64> = fs::read_to_string("inputs/day1/input.txt")
        .expect("Failed to read input")
        .lines()
        .map(|l| l.parse().expect("Failed to convert line to u64"))
        .collect();

    let mut result = 0;
    for i in 0..input.len() {
        if i < 3 {
            continue;
        }

        let a: u64 = input[i - 3..i].iter().sum();
        let b: u64 = input[i - 2..i + 1].iter().sum();

        if a < b {
            result += 1;
        }
    }

    println!("Day 1, part 2: {}", result);
}

fn main() {
    part1();
    part2();
}
