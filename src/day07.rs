pub fn part1() {
    let mut crabs: Vec<isize> = include_str!("../inputs/day7/input.txt")
        .trim()
        .split(',')
        .map(|str| str.parse().expect("a usize"))
        .collect();

    crabs.sort_unstable();

    let mean = crabs[crabs.len() / 2];

    let fuel: isize = crabs.iter().map(|c| (c - mean).abs()).sum();

    println!("Day 7, part 1: {}", fuel);
}

pub fn part2() {
    let crabs: Vec<i64> = include_str!("../inputs/day7/input.txt")
        .trim()
        .split(',')
        .map(|str| str.parse().expect("a usize"))
        .collect();

    let mut min = i64::max_value();
    let mut max = i64::min_value();
    for crab in &crabs {
        min = min.min(*crab);
        max = max.max(*crab);
    }

    let mut fuel = i64::max_value();
    for pos in min..(max + 1) {
        let mut pos_fuel = 0;
        for crab in &crabs {
            let distance = (pos - crab).abs();

            let crab_fuel = (distance * (1  + distance)) / 2;

            pos_fuel += crab_fuel;
        }

        fuel = fuel.min(pos_fuel);
    }

    println!("Day 7, part 2: {}", fuel);
}
