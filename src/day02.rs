use utils::{parse::*, part_impl};

pub enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

pub fn part_one(commands: &[Command]) -> i64 {
    let mut horizontal = 0i64;
    let mut depth = 0i64;

    for command in commands {
        match command {
            Command::Forward(offset) => horizontal += offset,
            Command::Up(offset) => depth -= offset,
            Command::Down(offset) => depth += offset,
        };
    }

    horizontal * depth
}

pub fn part_two(commands: &[Command]) -> i64 {
    let mut horizontal = 0i64;
    let mut depth = 0i64;
    let mut aim = 0i64;

    for command in commands {
        match command {
            Command::Forward(offset) => {
                horizontal += offset;
                depth += offset * aim;
            }
            Command::Up(offset) => aim -= offset,
            Command::Down(offset) => aim += offset,
        };
    }

    horizontal * depth
}

pub fn parse_input(input: &str) -> Vec<Command> {
    parse_lines2(
        input,
        " ",
        |direction| direction.chars().next().unwrap(),
        |offset| offset.parse::<i64>().unwrap(),
        |(direction, offset)| match direction {
            'f' => Command::Forward(offset),
            'd' => Command::Down(offset),
            'u' => Command::Up(offset),
            _ => unreachable!(),
        },
    )
}

#[allow(dead_code)]
pub fn parse_input_fast(input: &str) -> Vec<Command> {
    let mut values: Vec<Command> = Vec::with_capacity(2000);

    let bytes = input.as_bytes();
    let mut index: usize = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'f' => {
                index += 8;
                values.push(Command::Forward((bytes[index] - b'0') as i64));
                index += 2;
            }
            b'd' => {
                index += 5;
                values.push(Command::Down((bytes[index] - b'0') as i64));
                index += 2;
            }
            b'u' => {
                index += 3;
                values.push(Command::Up((bytes[index] - b'0') as i64));
                index += 2;
            }
            _ => break,
        }
    }
    values
}

part_impl! {
    part_one_test01: 150,
    part_one_problem: 2272262,
    part_two_test01: 900,
    part_two_problem: 2134882034,
}
