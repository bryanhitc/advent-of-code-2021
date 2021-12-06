// I lost my original solution, but this one's faster anyway, so let's just use this.
// Source: https://gitlab.com/AdventOfCode-SPetrie/aoc_2021/-/blob/master/src/day04.rs
#![allow(clippy::unusual_byte_groupings)]

use std::convert::From;
use utils::part_impl;

const ROW_0: u32 = 0b00000_00000_00000_00000_11111;
const ROW_1: u32 = 0b00000_00000_00000_11111_00000;
const ROW_2: u32 = 0b00000_00000_11111_00000_00000;
const ROW_3: u32 = 0b00000_11111_00000_00000_00000;
const ROW_4: u32 = 0b11111_00000_00000_00000_00000;
const COL_0: u32 = 0b10000_10000_10000_10000_10000;
const COL_1: u32 = 0b01000_01000_01000_01000_01000;
const COL_2: u32 = 0b00100_00100_00100_00100_00100;
const COL_3: u32 = 0b00010_00010_00010_00010_00010;
const COL_4: u32 = 0b00001_00001_00001_00001_00001;

const BOARD_SIZE: usize = 25;

#[derive(Clone, Copy, Debug)]
pub struct BingoBoard {
    won: bool,
    state: u32,
    numbers: [u8; BOARD_SIZE],
}

impl BingoBoard {
    pub fn update_won(&mut self) {
        match self.state {
            _ if (self.state & ROW_0) == ROW_0 => self.won = true,
            _ if (self.state & ROW_1) == ROW_1 => self.won = true,
            _ if (self.state & ROW_2) == ROW_2 => self.won = true,
            _ if (self.state & ROW_3) == ROW_3 => self.won = true,
            _ if (self.state & ROW_4) == ROW_4 => self.won = true,
            _ if (self.state & COL_0) == COL_0 => self.won = true,
            _ if (self.state & COL_1) == COL_1 => self.won = true,
            _ if (self.state & COL_2) == COL_2 => self.won = true,
            _ if (self.state & COL_3) == COL_3 => self.won = true,
            _ if (self.state & COL_4) == COL_4 => self.won = true,
            _ => {}
        }
    }

    pub fn select_num(&mut self, num: &u8) {
        let old_state = self.state;

        for i in 0..BOARD_SIZE {
            if self.numbers[i] == *num {
                self.state |= 1 << i;
            }
        }

        if self.state != old_state {
            self.update_won();
        }
    }

    pub fn sum_unused(&self) -> u64 {
        let mut sum = 0;

        for i in 0..BOARD_SIZE {
            if self.state & (1 << i) == 0 {
                sum += u64::from(self.numbers[i]);
            }
        }

        sum
    }
}

impl From<&[u8]> for BingoBoard {
    fn from(slice: &[u8]) -> Self {
        let mut board = BingoBoard {
            won: false,
            state: 0,
            numbers: [0; 25],
        };

        let mut index = 0;
        let mut number = 0;
        let mut value = 0;

        while index < slice.len() {
            if slice[index] != b' ' {
                value += slice[index] - b'0';
                value *= 10;
            }
            index += 1;
            value += slice[index] - b'0';
            board.numbers[number] = value;
            value = 0;
            number += 1;
            index += 2;
        }

        board
    }
}

pub fn part_one(input: &Input) -> u64 {
    let Input { nums, boards } = input;
    let mut owned_boards = boards.to_vec();

    for num in nums {
        for board in owned_boards.iter_mut() {
            board.select_num(num);
            if board.won {
                return board.sum_unused() * u64::from(*num);
            }
        }
    }

    0
}

pub fn part_two(input: &Input) -> u64 {
    let Input { nums, boards } = input;
    let mut owned_boards = boards.to_vec();

    for num in nums {
        for board in owned_boards.iter_mut() {
            board.select_num(num);
        }

        if owned_boards.len() == 1 && owned_boards[0].won {
            return owned_boards[0].sum_unused() * u64::from(*num);
        }

        owned_boards.retain(|board| !board.won);
    }

    0
}

#[derive(Debug)]
pub struct Input {
    nums: Vec<u8>,
    boards: Vec<BingoBoard>,
}

pub fn parse_input(input: &str) -> Input {
    let mut nums = Vec::with_capacity(100);
    let mut boards = Vec::with_capacity(100);

    let bytes = input.as_bytes();
    let mut value = 0;
    let mut index = 0;

    while index < bytes.len() {
        match bytes[index] {
            b'\n' => {
                nums.push(value);
                index += 2;
                break;
            }
            b',' => {
                nums.push(value);
                value = 0;
            }
            v => {
                value *= 10;
                value += v - b'0';
            }
        }
        index += 1;
    }

    // 15 bytes * 5 rows = 75 bytes/board + 1 byte for newline
    while index <= bytes.len() - 75 {
        let end = index + 75;
        boards.push(BingoBoard::from(&bytes[index..end]));
        index += 76;
    }

    Input { nums, boards }
}

part_impl! {
    part_one_test01: 4512,
    part_one_problem: 5685,
    part_two_test01: 1924,
    part_two_problem: 21070,
}
