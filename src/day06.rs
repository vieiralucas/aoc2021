fn solve(days: usize) -> usize {
    let mut fishes: [usize; 9] = include_str!("../inputs/day6/input.txt")
        .trim()
        .split(',')
        .map(|str| str.parse().expect("a usize"))
        .fold([0; 9], |mut fishes, fish: usize| {
            fishes[fish] += 1;
            fishes
        });

    for _ in 0..days {
        let ready = fishes[0];
        fishes[0] = 0;
        fishes.rotate_left(1);
        fishes[8] = ready;
        fishes[6] += ready;
    }

    fishes.iter().sum()
}

pub fn part1() {
    println!("Day 6, part 1: {}", solve(80));
}

pub fn part2() {
    println!("Day 6, part 2: {}", solve(256));
}
