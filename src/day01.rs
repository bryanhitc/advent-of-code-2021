type Depth = u16;

fn get_num_increased(depths: &[Depth], window_size: usize) -> u16 {
    depths
        .windows(window_size)
        .filter(|window| window.last() > window.first())
        // Use `fold` instead of `count` because the latter forces
        // `usize` which worsens benchmark results by ~400%. Maybe
        // `u16` triggers some automatic compiler vectorization
        // optimization due to reduced "width"?
        .fold(0u16, |acc, _| acc + 1)
}

pub fn part_one(depths: &[Depth]) -> u16 {
    get_num_increased(depths, 2)
}

pub fn part_two(depths: &[Depth]) -> u16 {
    get_num_increased(depths, 4)
}

pub fn parse_input(input: &str) -> Vec<Depth> {
    let mut depths = Vec::with_capacity(2000);

    let bytes = input.as_bytes();
    let mut value: Depth = 0;

    for byte in bytes {
        match *byte {
            b'\n' => {
                depths.push(value);
                value = 0;
            }
            v => {
                value *= 10;
                value += Depth::from(v - b'0');
            }
        }
    }

    depths
}

part_impl! {
    part_one_test01: 7,
    part_one_problem: 1532,
    part_two_test01: 5,
    part_two_problem: 1571,
}
