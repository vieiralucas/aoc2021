use std::fs;

fn day1_part1() {
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

fn day1_part2() {
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

fn day2_part1() {
    let mut hor = 0;
    let mut depth = 0;

    for line in fs::read_to_string("inputs/day2/input.txt")
        .expect("Failed to read input")
        .lines()
    {
        let mut words = line.split(" ");
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

fn day2_part2() {
    let mut hor = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in fs::read_to_string("inputs/day2/input.txt")
        .expect("Failed to read input")
        .lines()
    {
        let mut words = line.split(" ");
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

fn main() {
    day1_part1();
    day1_part2();

    day2_part1();
    day2_part2();
}
