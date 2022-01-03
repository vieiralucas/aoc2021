pub fn part1() {
    let mut fishes: Vec<usize> = include_str!("../inputs/day6/input.txt")
        .trim()
        .split(',')
        .map(|str| str.parse().expect("a usize"))
        .collect();

    let mut next_fishes: Vec<usize> = fishes.clone();

    for _ in 0..80 {
        for (i, fish) in fishes.iter().enumerate() {
            if *fish == 0 {
                next_fishes[i] = 6;
                next_fishes.push(8);
            } else {
                next_fishes[i] = fish - 1;
            }
        }

        fishes = next_fishes.clone();
    }

    println!("Day 6, part 1: {}", fishes.len());
}

pub fn part2() {
    let _fishes: Vec<usize> = include_str!("../inputs/day6/sample.txt")
        .trim()
        .split(',')
        .map(|str| str.parse().expect("a usize"))
        .collect();

    println!("Day 6, part 2: {}", 0);
}

