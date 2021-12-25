use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const BOARD_SIZE: usize = 5;

#[derive(Default, Debug, Clone)]
struct Board {
    board_map: HashMap<u32, (usize, usize)>,
    row_count: [u8; BOARD_SIZE],
    col_count: [u8; BOARD_SIZE],
    board_won: bool,
}

impl Board {
    fn mark_number(&mut self, num: &u32) -> bool {
        if let Some((row, col)) = self.board_map.remove(num) {
            self.row_count[row] += 1;
            self.col_count[col] += 1;

            if self.row_count[row] == BOARD_SIZE as u8 || self.col_count[col] == BOARD_SIZE as u8 {
                self.board_won = true;
                return true;
            }
        }

        false
    }

    fn calc_board_sum(&self) -> u32 {
        self.board_map.keys().sum()
    }

    fn board_won(&self) -> bool {
        self.board_won
    }
}

fn part_1(numbers: &[u32], mut boards: Vec<Board>) {
    'outer: for num in numbers {
        for board in boards.iter_mut() {
            if board.mark_number(num) {
                let board_sum = board.calc_board_sum();
                println!(
                    "Part 1 - Winning Number: {} - Board Sum: {} - Answer: {}",
                    num,
                    board_sum,
                    num * board_sum
                );

                break 'outer;
            }
        }
    }
}

fn part_2(numbers: &[u32], mut boards: Vec<Board>) {
    let mut last_score = 0;
    for num in numbers {
        for board in boards.iter_mut() {
            if !board.board_won() && board.mark_number(num) {
                let board_sum = board.calc_board_sum();
                last_score = board_sum * num;
            }
        }
    }

    println!("Part 2 - Answer: {}", last_score);
}

fn main() {
    let mut reader = BufReader::new(File::open("day_4/input.txt").unwrap());

    let mut number_string = String::new();
    reader.read_line(&mut number_string).unwrap();
    let number_string = number_string.trim();

    let numbers: Vec<u32> = number_string
        .split(',')
        .map(|num_str| num_str.parse::<u32>().unwrap())
        .collect();

    // Skip the new line after the bingo numbers
    let mut number_string = String::new();
    reader.read_line(&mut number_string).unwrap();
    number_string.clear();

    let mut boards = Vec::new();
    let mut local_board = Board::default();
    while let Ok(bytes_read) = reader.read_line(&mut number_string) {
        if bytes_read == 1 {
            let board_numbers: Vec<u32> = number_string
                .split_whitespace()
                .map(|num_str| num_str.parse::<u32>().unwrap())
                .collect();

            for (idx, num) in board_numbers.into_iter().enumerate() {
                let row = idx / BOARD_SIZE;
                let col = idx % BOARD_SIZE;

                local_board.board_map.insert(num, (row, col));
            }

            boards.push(std::mem::take(&mut local_board));
            number_string.clear();
        } else if bytes_read == 0 {
            break;
        }
    }

    part_1(&numbers, boards.clone());
    part_2(&numbers, boards);
}
