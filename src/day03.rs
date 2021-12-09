use utils::part_impl;

use std::cmp::Ordering;

type BinaryNum = u32;
type Answer = u64;

#[inline]
fn is_bit_set(input: BinaryNum, pos: BinaryNum) -> bool {
    ((input >> pos) & 1) == 1
}

#[inline]
fn one_bit_frequency(nums: &[BinaryNum], pos: BinaryNum) -> Ordering {
    let count = nums.iter().fold(0usize, |total, &num|
        total + is_bit_set(num, pos) as usize
    );

    count.cmp(&(nums.len() - count))
}

#[inline]
fn process_pos(
    nums: &mut Vec<BinaryNum>,
    pos: BinaryNum,
    should_retain_bit: impl Fn(BinaryNum) -> bool,
) -> Option<BinaryNum> {
    match one_bit_frequency(nums, pos) {
        Ordering::Greater | Ordering::Equal => {
            nums.retain(|v| should_retain_bit(*v));
        }
        Ordering::Less => {
            nums.retain(|v| !should_retain_bit(*v));
        }
    }

    (nums.len() == 1).then(|| nums[0])
}

pub fn part_one(input: &Input) -> Answer {
    let nums = input.nums.as_slice();
    let num_digits = input.num_digits;

    let gamma = {
        let mut gamma = 0;
        for pos in (0..num_digits).rev() {
            gamma <<= 1;
            match one_bit_frequency(nums, pos) {
                Ordering::Greater | Ordering::Equal => gamma += 1,
                Ordering::Less => {}
            }
        }
        gamma
    };
    let epsilon = gamma
        ^ (BinaryNum::MAX >> ((std::mem::size_of::<BinaryNum>() * 8) as BinaryNum - num_digits));

    Answer::from(gamma) * Answer::from(epsilon)
}

pub fn part_two(input: &Input) -> Answer {
    let nums = input.nums.as_slice();
    let num_digits = input.num_digits;

    let mut o2 = 0;
    let mut co2 = 0;
    let mut num_finished = 0;

    let mut o2_list = Vec::<BinaryNum>::from(nums);
    let mut co2_list = Vec::<BinaryNum>::from(nums);

    for pos in (0..num_digits).rev() {
        if let Some(o2_value) = process_pos(&mut o2_list, pos, |bit| is_bit_set(bit, pos)) {
            o2 = o2_value;
            num_finished += 1;
        }

        if let Some(co2_value) = process_pos(&mut co2_list, pos, |bit| !is_bit_set(bit, pos)) {
            co2 = co2_value;
            num_finished += 1;
        }

        if num_finished == 2 {
            break;
        }
    }

    Answer::from(o2) * Answer::from(co2)
}

#[derive(Debug)]
pub struct Input {
    nums: Vec<BinaryNum>,
    num_digits: BinaryNum,
}

pub fn parse_input(input: &str) -> Input {
    let mut nums = Vec::<BinaryNum>::with_capacity(2000);

    let bytes = input.as_bytes();
    let num_digits = bytes.iter().take_while(|&&byte| byte != b'\n').count() as BinaryNum;

    let mut value: BinaryNum = 0;
    for byte in bytes {
        match *byte {
            b'\n' => {
                nums.push(value);
                value = 0;
            }
            v => {
                value <<= 1;
                value |= BinaryNum::from(v - b'0');
            }
        }
    }

    Input { nums, num_digits }
}

part_impl! {
    part_one_test01: 198,
    part_one_problem: 852500,
    part_two_test01: 230,
    part_two_problem: 1007985,
}
