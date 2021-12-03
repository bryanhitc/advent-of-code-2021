use utils::{parse::*, part_impl};

// digits - 1 technically
const fn bit_mask(num_digits: usize) -> u32 {
    let mut mask = 0;
    let mut i = 0;

    while i < num_digits {
        mask |= 1 << i;
        i += 1;
    }

    mask
}

const BIT_MASKS: [u32; 32] = [
    0,
    bit_mask(1),
    bit_mask(2),
    bit_mask(3),
    bit_mask(4),
    bit_mask(5),
    bit_mask(6),
    bit_mask(7),
    bit_mask(8),
    bit_mask(9),
    bit_mask(10),
    bit_mask(11),
    bit_mask(12),
    bit_mask(13),
    bit_mask(14),
    bit_mask(15),
    bit_mask(16),
    bit_mask(17),
    bit_mask(18),
    bit_mask(19),
    bit_mask(20),
    bit_mask(21),
    bit_mask(22),
    bit_mask(23),
    bit_mask(24),
    bit_mask(25),
    bit_mask(26),
    bit_mask(27),
    bit_mask(28),
    bit_mask(29),
    bit_mask(30),
    bit_mask(31),
];

pub fn gamma(nums: &[&[u8]], num_digits: usize) -> u32 {
    let mut values = vec![0i32; num_digits];

    for &num in nums {
        for (i, &bit) in num.iter().enumerate() {
            values[i] -= match bit {
                b'0' => -1,
                _ => 1,
            }
        }
    }

    values
        .into_iter()
        .enumerate()
        .fold(0u32, |result, (i, num_ones)| match num_ones {
            0.. => result | 1 << (num_digits - i - 1),
            _ => result,
        })
}

pub fn part_one(nums: &[&[u8]]) -> u32 {
    let num_digits = nums[0].len();
    let gamma = gamma(nums, num_digits);
    let epsilon = !gamma & BIT_MASKS[num_digits];

    gamma * epsilon
}

fn rating<'input>(
    nums: &'input [&[u8]],
    num_digits: usize,
    zeros: &mut Vec<&'input [u8]>,
    ones: &mut Vec<&'input [u8]>,
    use_zeros: impl Fn(usize, usize) -> bool,
) -> u32 {
    let mut nums = nums.iter().copied().collect::<Vec<_>>();

    for i in 0..num_digits {
        if nums.len() == 1 {
            break;
        }

        zeros.clear();
        ones.clear();

        for &num in &nums {
            match num[i] {
                b'0' => zeros.push(num),
                _ => ones.push(num),
            }
        }

        if use_zeros(zeros.len(), ones.len()) {
            std::mem::swap(&mut nums, zeros);
        } else {
            std::mem::swap(&mut nums, ones);
        }
    }

    u32::from_str_radix(std::str::from_utf8(nums[0]).unwrap(), 2).unwrap()
}

fn oxygen_rating<'input>(
    nums: &'input [&[u8]],
    num_digits: usize,
    zeros: &mut Vec<&'input [u8]>,
    ones: &mut Vec<&'input [u8]>,
) -> u32 {
    rating(nums, num_digits, zeros, ones, |num_zeros, num_ones| {
        num_zeros > num_ones
    })
}

fn co2_rating<'input>(
    nums: &'input [&[u8]],
    num_digits: usize,
    zeros: &mut Vec<&'input [u8]>,
    ones: &mut Vec<&'input [u8]>,
) -> u32 {
    rating(nums, num_digits, zeros, ones, |num_zeros, num_ones| {
        num_zeros <= num_ones
    })
}

pub fn part_two(nums: &[&[u8]]) -> u32 {
    let num_digits = nums[0].len();
    let mut zeros = Vec::with_capacity(nums.len());
    let mut ones = Vec::with_capacity(nums.len());

    let oxygen = oxygen_rating(nums, num_digits, &mut zeros, &mut ones);
    let co2 = co2_rating(nums, num_digits, &mut zeros, &mut ones);

    oxygen * co2
}

pub fn parse_input(input: &str) -> Vec<&[u8]> {
    input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>()
}

part_impl! {
    part_one_test01: 198,
    part_one_problem: 852500,
    part_two_test01: 230,
    part_two_problem: 1007985,
}
