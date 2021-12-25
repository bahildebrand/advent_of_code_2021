use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_bits(values: &[u32], bit_mask: u32) -> usize {
    let mut one_count = 0;
    for value in values {
        if *value & bit_mask != 0 {
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

        if one_count > values.len() - one_count {
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

fn life_support_rating(
    mut values: Vec<u32>,
    comparison_function: &dyn Fn(usize, usize) -> bool,
) -> u32 {
    let mut bit_mask: u32 = 0x800;
    for _ in 0..12 {
        let one_count = count_bits(&values, bit_mask);

        if comparison_function(one_count, values.len()) {
            values.retain(|value| value & bit_mask != 0);
        } else {
            values.retain(|value| value & bit_mask == 0);
        }

        bit_mask >>= 1;

        if values.len() == 1 {
            break;
        }
    }

    values[0]
}

fn part_2(values: Vec<u32>) {
    let oxygen_comparison = |one_count, values_len| one_count >= values_len - one_count;
    let oxygen_rating = life_support_rating(values.clone(), &oxygen_comparison);

    let co2_comparison = |one_count, values_len| one_count < values_len - one_count;
    let co2_rating = life_support_rating(values, &co2_comparison);

    println!(
        "Part 2 - Oxygen: {} - CO2: {} - Life Support: {}",
        oxygen_rating,
        co2_rating,
        oxygen_rating * co2_rating
    );
}

fn main() {
    let reader = BufReader::new(File::open("day_3/input.txt").unwrap());

    let values: Vec<u32> = reader
        .lines()
        .map(|line| u32::from_str_radix(&line.unwrap(), 2).unwrap())
        .collect();

    part_1(&values);
    part_2(values);
}
