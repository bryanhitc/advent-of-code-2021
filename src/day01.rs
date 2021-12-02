// use itertools::Itertools;
use utils::{parse::*, part_impl};

pub fn parse_input(input: &str) -> Vec<u32> {
    default_parse_lines(input)
}

// Invariant: input.len() > window_size
fn get_num_increased(depths: &[u32], window_size: usize) -> u32 {
    let mut increases: u32 = 0;
    for index in window_size..depths.len() {
        if depths[index] > depths[index - window_size] {
            increases += 1;
        }
    }
    increases
}

pub fn part_one(depths: &[u32]) -> u32 {
    get_num_increased(depths, 1)
}

pub fn part_two(depths: &[u32]) -> u32 {
    get_num_increased(depths, 3)
}

// pub fn part_one(depths: &[u32]) -> usize {
//     depths
//         .iter()
//         .tuple_windows()
//         .filter(|(&prev, &curr)| curr > prev)
//         .count()
// }

// pub fn part_two(depths: &[u32]) -> usize {
//     // This is likely slower than it needs to be since the window sum
//     // can be calculated as prevWindow sum - evictedValue + newValue.
//     // LLVM might already be doing this and/or using avx, though.
//     // The latter would require lto with target-cpu=native or some avx flag.
//     let window_sums = depths.windows(3).map(|window| window.iter().sum::<u32>());

//     window_sums
//         .tuple_windows()
//         .filter(|(prev, curr)| curr > prev)
//         .count()
// }

part_impl! {
    part_one_test01: 7,
    part_one_problem: 1532,
    part_two_test01: 5,
    part_two_problem: 1571,
}
