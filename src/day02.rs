type Magnitude = i32;
type Answer = i32;

#[derive(Clone, Copy, Debug)]
pub enum Command {
    Forward(Magnitude),
    Down(Magnitude),
    Up(Magnitude),
}

pub fn part_one(commands: &[Command]) -> Answer {
    let mut horizontal = 0;
    let mut depth = 0;

    for command in commands {
        match command {
            Command::Forward(offset) => horizontal += offset,
            Command::Up(offset) => depth -= offset,
            Command::Down(offset) => depth += offset,
        };
    }

    horizontal * depth
}

pub fn part_two(commands: &[Command]) -> Answer {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands {
        match command {
            Command::Forward(offset) => {
                horizontal += *offset;
                depth += (*offset) * aim;
            }
            Command::Up(offset) => aim -= *offset,
            Command::Down(offset) => aim += *offset,
        };
    }

    horizontal * depth
}

pub fn parse_input(input: &str) -> Vec<Command> {
    let mut commands = Vec::<Command>::with_capacity(2000);

    let bytes = input.as_bytes();
    let mut index: usize = 0;

    while index < bytes.len() {
        match bytes[index] {
            b'f' => {
                index += 8;
                commands.push(Command::Forward((bytes[index] - b'0') as Magnitude));
                index += 2;
            }
            b'd' => {
                index += 5;
                commands.push(Command::Down((bytes[index] - b'0') as Magnitude));
                index += 2;
            }
            b'u' => {
                index += 3;
                commands.push(Command::Up((bytes[index] - b'0') as Magnitude));
                index += 2;
            }
            _ => break,
        }
    }

    commands
}

part_impl! {
    part_one_test01: 150,
    part_one_problem: 2272262,
    part_two_test01: 900,
    part_two_problem: 2134882034,
}
