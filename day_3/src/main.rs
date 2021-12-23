use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_bits(values: &[u32], bit_mask: u32) -> usize {
    let mut one_count = 0;
    for value in values {
        if *value & bit_mask == bit_mask {
            one_count += 1;
        }
    }

    one_count
}

fn part_1(values: &[u32]) {
    let mut delta_rate = 0;
    let mut epsilon_rate = 0;

    let mut rate_mask: u32 = 0x800;
    for _ in 0..12 {
        let one_count = count_bits(&values, rate_mask);

        if one_count > values.len() / 2 {
            delta_rate |= rate_mask;
        } else {
            epsilon_rate |= rate_mask;
        }

        rate_mask >>= 1;
    }

    println!(
        "Part 1 - Delta: {} - Epsilon: {} - Total: {}",
        delta_rate,
        epsilon_rate,
        delta_rate * epsilon_rate
    );
}

fn main() {
    let reader = BufReader::new(File::open("day_3/input.txt").unwrap());

    let values: Vec<u32> = reader
        .lines()
        .map(|line| u32::from_str_radix(&line.unwrap(), 2).unwrap())
        .collect();

    part_1(&values);
}
