type Depth = u16;

// Invariant: input.len() > window_size
fn get_num_increased(depths: &[Depth], window_size: usize) -> Depth {
    let mut increases: Depth = 0;

    for index in window_size..depths.len() {
        if depths[index] > depths[index - window_size] {
            increases += 1;
        }
    }

    increases
}

pub fn part_one(depths: &[Depth]) -> Depth {
    get_num_increased(depths, 1)
}

pub fn part_two(depths: &[Depth]) -> Depth {
    get_num_increased(depths, 3)
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
                value += (v - b'0') as Depth;
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
