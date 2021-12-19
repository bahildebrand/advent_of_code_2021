use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_1(values: &[u64]) {
    let mut gt_count = 0;
    for idx in 1..values.len() {
        if values[idx] > values[idx - 1] {
            gt_count += 1;
        }
    }

    println!("Part 1: {}", gt_count);
}

fn part_2(values: &[u64]) {
    let mut gt_count = 0;
    let mut prev_window = None;
    let windows = values.windows(3);
    for window in windows {
        let cur_window: u64 = window.iter().sum();
        if let Some(prev_window) = prev_window {
            if cur_window > prev_window {
                gt_count += 1;
            }
        }
        prev_window = Some(cur_window);
    }

    println!("Part 2: {}", gt_count);
}

fn main() {
    let reader = BufReader::new(File::open("day_1/input.txt").unwrap());

    let values: Vec<u64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<u64>().unwrap())
        .collect();

    if values.is_empty() {
        println!("0");
    }

    part_1(&values);
    part_2(&values);
}
