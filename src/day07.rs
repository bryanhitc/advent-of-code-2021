#[derive(Debug)]
pub struct Input(Vec<u16>);

#[allow(dead_code)]
pub mod impl07 {
    use super::Input;

    fn calculate(crabs: &[u16], cost: impl Fn(u16, u16) -> usize) -> usize {
        // brute force; idc
        let min_pos = *crabs.iter().min().unwrap();
        let max_pos = *crabs.iter().max().unwrap();

        (min_pos..=max_pos)
            .map(|pos| crabs.iter().map(|&crab| cost(pos, crab)).sum())
            .min()
            .unwrap_or_default()
    }

    fn distance(pos: u16, crab: u16) -> usize {
        (crab as i32 - pos as i32).abs() as usize
    }

    pub fn part_one(crabs: &[u16]) -> usize {
        calculate(&crabs.0, distance)
    }

    pub fn part_two(crabs: &Input) -> usize {
        calculate(&crabs.0, |pos, crab| {
            let num = distance(pos, crab);
            (num * (num + 1)) / 2
        })
    }

    pub fn parse_input(input: &str) -> Input {
        Input(
            input
                .trim()
                .split(',')
                .map(|num| num.parse::<u16>().unwrap())
                .collect::<Vec<_>>(),
        )
    }
}

#[allow(dead_code)]
pub mod implopt07 {
    use super::Input;

    fn median(crabs: &[u16]) -> u16 {
        match crabs.len() % 2 {
            0 => (crabs[crabs.len() / 2 - 1] + crabs[crabs.len() / 2]) / 2,
            1 => crabs[crabs.len() / 2],
            _ => unreachable!(),
        }
    }

    fn mean(crabs: &[u16]) -> u16 {
        (crabs
            .iter()
            .fold(usize::default(), |total, &crab| total + crab as usize)
            / crabs.len() as usize) as u16
    }

    fn calculate(crabs: &[u16], cost: impl Fn(u16) -> usize) -> usize {
        crabs.iter().map(|&crab| cost(crab)).sum()
    }

    fn distance(pos: u16, crab: u16) -> usize {
        // can make this branchless by partitioning <= pos and > pos
        (crab as i32 - pos as i32).abs() as usize
    }

    pub fn part_one(crabs: &[u16]) -> usize {
        let optimal_pos = median(crabs);
        calculate(crabs, |crab| distance(optimal_pos, crab))
    }

    pub fn part_two(crabs: &[u16]) -> usize {
        let mean = mean(crabs);
        let result = (mean..=mean + 1)
            .map(|pos| {
                calculate(crabs, |crab| {
                    // summation for 0..num but leave / 2 for the end
                    let num = distance(pos, crab);
                    num * (num + 1)
                })
            })
            .min()
            .unwrap_or_default();

        result / 2
    }

    pub fn parse_input(input: &str) -> Vec<u16> {
        let mut crabs = Vec::with_capacity(1000);

        let bytes = input.as_bytes();
        let mut value = 0;

        for &byte in bytes {
            match byte {
                b',' | b'\n' => {
                    crabs.push(value);
                    value = 0;
                }
                // digit 0-9
                _ => {
                    value *= 10;
                    value += u16::from(byte - b'0');
                }
            }
        }

        crabs.sort_unstable();
        crabs
    }
}

use implopt07::*;

part_impl! {
    part_one_test01: 37,
    part_one_problem: 352254,
    part_two_test01: 168,
    part_two_problem: 99053143,
}
