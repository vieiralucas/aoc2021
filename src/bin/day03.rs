use std::fs;

fn part1() {
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

fn part2() {
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
            } else if bits[i] == '1' {
                next_bits.push(bits.clone());
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
            } else if bits[i] == '1' {
                next_bits.push(bits.clone());
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
    part1();
    part2();
}
