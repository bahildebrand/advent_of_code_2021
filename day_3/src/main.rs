use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("day_3/input.txt").unwrap());

    let values: Vec<u32> = reader
        .lines()
        .map(|line| u32::from_str_radix(&line.unwrap(), 2).unwrap())
        .collect();

    println!("First value: {}", values[0]);

    let mut delta_rate = 0;
    let mut epsilon_rate = 0;

    let mut rate_mask: u32 = 0x800;
    for _ in 0..12 {
        let mut one_count = 0;
        for value in &values {
            if *value & rate_mask == rate_mask {
                one_count += 1;
            }
        }

        println!("Ones: {}, Zeroes: {}", one_count, values.len() - one_count);
        if one_count > values.len() / 2 {
            delta_rate |= rate_mask;
        } else {
            epsilon_rate |= rate_mask;
        }

        println!("Delta: {} - Epsilon: {}", delta_rate, epsilon_rate);
        rate_mask >>= 1;
    }

    println!(
        "Delta: {} - Epsilon: {} - Total: {}",
        delta_rate,
        epsilon_rate,
        delta_rate * epsilon_rate
    );
}
