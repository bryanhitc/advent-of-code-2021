// use itertools::Itertools;
use utils::part_impl;

pub mod day08impl {
    use std::{collections::BTreeMap, hash::Hash, str::Split};

    type Input<'a> = Vec<(Split<'a, char>, Result<'a>)>;

    enum Digit<'a> {
        Zero(&'a str),
        One(&'a str),
        Two(&'a str),
        Three(&'a str),
        Four(&'a str),
        Five(&'a str),
        Six(&'a str),
        Seven(&'a str),
        Eight(&'a str),
        Nine(&'a str),
    }

    pub struct Result<'a> {
        numbers: [&'a str; 4],
    }

    impl<'a> Result<'a> {
        pub fn calculate(&'a self, map: &[(char, char)]) -> usize {
            let mut result = 0;
            result += Digit::parse_result(self.numbers[0], map).to_num() * 1000;
            result += Digit::parse_result(self.numbers[1], map).to_num() * 100;
            result += Digit::parse_result(self.numbers[2], map).to_num() * 10;
            result += Digit::parse_result(self.numbers[3], map).to_num();
            result
        }
    }

    impl<'a> Digit<'a> {
        pub fn parse_known(segments: &'a str) -> Option<Self> {
            match segments.len() {
                2 => Some(Self::One(segments)),
                3 => Some(Self::Seven(segments)),
                4 => Some(Self::Four(segments)),
                7 => Some(Self::Eight(segments)),
                _ => None,
            }
        }

        pub fn parse_result(segments: &'a str, map: &[(char, char)]) -> Self {
            let hash = segments
                .chars()
                .map(|char| {
                    map.iter().cloned()
                        .find_map(|(letter, val)| {
                            (letter == char).then(|| val.to_digit(10).unwrap() as u8)
                        })
                        .unwrap()
                })
                .fold(0u16, |acc, num| acc | (1 << num));

            // match hash {
            //     "012456" => Digit::Zero(segments),
            //     "25" => Digit::One(segments),
            //     "02346" => Digit::Two(segments),
            //     "02356" => Digit::Three(segments),
            //     "1235" => Digit::Four(segments),
            //     "01356" => Digit::Five(segments),
            //     "013456" => Digit::Six(segments),
            //     "025" => Digit::Seven(segments),
            //     "0123456" => Digit::Eight(segments),
            //     "012356" => Digit::Nine(segments),
            //     _ => unreachable!(),
            // }

            match hash {
                0b01110111 => Digit::Zero(segments),
                0b00100100 => Digit::One(segments),
                0b01011101 => Digit::Two(segments),
                0b00110111 => Digit::Three(segments),
                0b00101110 => Digit::Four(segments),
                0b01101011 => Digit::Five(segments),
                0b01101001 => Digit::Six(segments),
                0b00100101 => Digit::Seven(segments),
                0b01111111 => Digit::Eight(segments),
                0b01101111 => Digit::Nine(segments),
                _ => unreachable!(),
            }
        }

        pub const fn to_num(&self) -> usize {
            match self {
                Zero => 0,
                One => 1,
                Two => 2,
                Three => 3,
                Four => 4,
                Five => 5,
                Six => 6,
                Seven => 7,
                Eight => 8,
                Nine => 9,
            }
        }
    }

    pub fn part_one(input: &Input) -> usize {
        input
            .iter()
            .map(|(_wires, results)| {
                results
                    .numbers
                    .iter()
                    .cloned()
                    .filter_map(Digit::parse_known)
                    .count()
            })
            .sum()
    }

    pub fn part_two(lines: &Input) -> usize {
        // mapping
        //  0000
        // 1    2
        // 1    2
        //  3333
        // 4    5
        // 4    5
        //  6666

        // let wire_to_segment_id = Vec::<(char, char)>::with_capacity(10);
        // let mut ans = 0;

        // for (wires, result) in lines {
        //     let known_digits = wires.filter_map(Digit::parse_known);

        //     for digit in known_digits {

        //     }

        //     ans += result.calculate(&wire_to_segment_id);
        //     wire_to_segment_id.clear();
        // }

        // ans
        // todo!()
        0
    }

    pub fn parse_input(input: &str) -> Input {
        input
            .lines()
            .map(|line| {
                let mut parts = line.split(" | ");
                let wires = parts.next().unwrap().split(' ');

                let mut results = parts.next().unwrap().split(' ');
                let result = Result {
                    numbers: [
                        results.next().unwrap(),
                        results.next().unwrap(),
                        results.next().unwrap(),
                        results.next().unwrap(),
                    ],
                };

                (wires, result)
            })
            .collect::<Vec<_>>()
    }
}

use day08impl::*;

part_impl! {
    part_one_test01: 26,
    part_one_problem: 514,
    // part_two_test01: 5353,

}
