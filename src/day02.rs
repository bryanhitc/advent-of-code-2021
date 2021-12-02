use utils::{parse::*, part_impl};

pub enum Direction {
    Forward,
    Down,
    Up,
}

pub struct Command {
    direction: Direction,
    offset: i64,
}

pub fn part_one(commands: &[Command]) -> i64 {
    let mut horizontal = 0i64;
    let mut depth = 0i64;

    for command in commands {
        match command.direction {
            Direction::Forward => horizontal += command.offset,
            Direction::Up => depth -= command.offset,
            Direction::Down => depth += command.offset,
        };
    }

    horizontal * depth
}

pub fn part_two(commands: &[Command]) -> i64 {
    let mut horizontal = 0i64;
    let mut depth = 0i64;
    let mut aim = 0i64;

    for command in commands {
        match command.direction {
            Direction::Forward => {
                horizontal += command.offset;
                depth += command.offset * aim;
            }
            Direction::Up => aim -= command.offset,
            Direction::Down => aim += command.offset,
        };
    }

    horizontal * depth
}

pub fn parse_input(input: &str) -> Vec<Command> {
    parse_lines2(
        input,
        " ",
        |direction| match direction.chars().next().unwrap() {
            'f' => Direction::Forward,
            'd' => Direction::Down,
            'u' => Direction::Up,
            _ => panic!("invalid direction"),
        },
        |offset| offset.parse::<i64>().unwrap(),
        |(direction, offset)| Command { direction, offset },
    )
}

part_impl! {
    part_one_test01: 150,
    part_one_problem: 2272262,
    part_two_test01: 900,
    part_two_problem: 2134882034,
}
