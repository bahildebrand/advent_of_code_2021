use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_1(commands: &[String]) {
    let mut position = (0, 0);
    for command in commands {
        let subs: Vec<&str> = command.split(" ").collect();
        match subs[0] {
            "forward" => position.0 += subs[1].parse::<u64>().unwrap(),
            "up" => position.1 -= subs[1].parse::<u64>().unwrap(),
            "down" => position.1 += subs[1].parse::<u64>().unwrap(),
            _ => panic!("Invalid command {}", subs[0]),
        }
    }

    println!("{}", position.0 * position.1);
}

fn part_2(commands: &[String]) {
    let mut position = (0, 0);
    let mut aim = 0;
    for command in commands {
        let subs: Vec<&str> = command.split(" ").collect();
        match subs[0] {
            "forward" => {
                let forward = subs[1].parse::<i64>().unwrap();
                position.0 += forward;
                position.1 += forward * aim;
            }
            "up" => aim -= subs[1].parse::<i64>().unwrap(),
            "down" => aim += subs[1].parse::<i64>().unwrap(),
            _ => panic!("Invalid command {}", subs[0]),
        }
    }

    println!("{}", position.0 * position.1);
}

fn main() {
    let reader = BufReader::new(File::open("day_2/input.txt").unwrap());

    let commands: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    if commands.is_empty() {
        println!("0");
    }

    part_1(&commands);
    part_2(&commands);
}
