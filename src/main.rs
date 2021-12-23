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

fn day3_part1() {
    let mut ocurrences = vec![];
    let mut line_count = 0;

    for line in fs::read_to_string("inputs/day3/input.txt")
        .expect("Failed to read input")
        .lines()
    {
        line_count += 1;
        for (i, c) in line.chars().enumerate() {
            if i >= ocurrences.len() {
                ocurrences.push(0);
            }

            if c == '1' {
                ocurrences[i] += 1;
            }
        }
    }

    let mut gamma_bits = vec!['0'; ocurrences.len()];
    let mut epsilon_bits = vec!['0'; ocurrences.len()];

    for i in 0..ocurrences.len() {
        if ocurrences[i] > line_count / 2 {
            gamma_bits[i] = '1';
        } else {
            epsilon_bits[i] = '1';
        }
    }

    let gamma = u32::from_str_radix(&String::from_iter(gamma_bits), 2)
        .expect("Failed to parse bits gamma_bits");
    let epsilon = u32::from_str_radix(&String::from_iter(epsilon_bits), 2)
        .expect("Failed to parse bits epsilon_bits");

    println!("Day 3, part 1: {}", gamma * epsilon);
}

fn day3_part2() {
    let mut bits: Vec<Vec<char>> = vec![];

    for line in fs::read_to_string("inputs/day3/input.txt")
        .expect("Failed to read input")
        .lines()
    {
        bits.push(line.chars().clone().collect());
    }

    let mut oxygen_bits = bits.clone();
    let mut i = 0;
    while oxygen_bits.len() > 1 {
        let mut ones = 0;
        let mut zeroes = 0;

        for bits in &oxygen_bits {
            if bits[i] == '1' {
                ones += 1;
            } else {
                zeroes += 1
            }
        }

        let mut next_bits: Vec<Vec<char>> = Vec::new();

        for bits in &oxygen_bits {
            if zeroes > ones {
                if bits[i] == '0' {
                    next_bits.push(bits.clone());
                }
            } else {
                if bits[i] == '1' {
                    next_bits.push(bits.clone());
                }
            }
        }

        oxygen_bits = next_bits;

        i += 1;
    }

    i = 0;
    let mut co2_bits = bits.clone();
    while co2_bits.len() > 1 {
        let mut ones = 0;
        let mut zeroes = 0;

        for bits in &co2_bits {
            if bits[i] == '1' {
                ones += 1;
            } else {
                zeroes += 1
            }
        }

        let mut next_bits: Vec<Vec<char>> = Vec::new();

        for bits in &co2_bits {
            if zeroes <= ones {
                if bits[i] == '0' {
                    next_bits.push(bits.clone());
                }
            } else {
                if bits[i] == '1' {
                    next_bits.push(bits.clone());
                }
            }
        }

        co2_bits = next_bits;

        i += 1;
    }

    assert!(oxygen_bits.len() == 1);
    assert!(co2_bits.len() == 1);

    let oxygen = u32::from_str_radix(&String::from_iter(&oxygen_bits[0]), 2)
        .expect("Failed to parse oxygen_bits");
    let co2 = u32::from_str_radix(&String::from_iter(&co2_bits[0]), 2)
        .expect("Failed to parse bits co2_bits");

    println!("Day 3, part 2: {}", oxygen * co2);
}

fn main() {
    day1_part1();
    day1_part2();
    println!("-------------------------");

    day2_part1();
    day2_part2();
    println!("-------------------------");

    day3_part1();
    day3_part2();
    println!("-------------------------");
}
