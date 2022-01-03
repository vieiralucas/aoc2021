use std::fs;

pub fn part1() {
    let mut hor = 0;
    let mut depth = 0;

    for line in fs::read_to_string("inputs/day2/input.txt")
        .expect("Failed to read input")
        .lines()
    {
        let mut words = line.split(' ');
        let dir = words.next().expect("Direction not found in line");
        let amount: u64 = words
            .next()
            .map(|w| w.parse().expect("Failed to parse amount to u64"))
            .expect("Amount not found in line");

        match dir {
            "forward" => hor += amount,
            "up" => depth -= amount,
            "down" => depth += amount,
            _ => unreachable!(),
        }
    }

    println!("Day 2, part 1: {}", hor * depth);
}

pub fn part2() {
    let mut hor = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in fs::read_to_string("inputs/day2/input.txt")
        .expect("Failed to read input")
        .lines()
    {
        let mut words = line.split(' ');
        let dir = words.next().expect("Direction not found in line");
        let amount: u64 = words
            .next()
            .map(|w| w.parse().expect("Failed to parse amount to u64"))
            .expect("Amount not found in line");

        match dir {
            "forward" => {
                hor += amount;
                depth += amount * aim;
            }
            "up" => aim -= amount,
            "down" => aim += amount,
            _ => unreachable!(),
        }
    }

    println!("Day 2, part 2: {}", hor * depth);
}
