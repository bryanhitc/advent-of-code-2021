// Naive solution, so it's slow
type Num = u16;

#[inline(always)]
fn increment(num: &mut Num) {
    *num += 1;
}

#[inline(always)]
fn decrement(num: &mut Num) {
    *num -= 1;
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: Num,
    y: Num,
}

#[derive(Clone, Copy, Debug)]
pub enum Path {
    Horizontal((Num, Num, Num)),
    Vertical((Num, Num, Num)),
    Diagonal((Point, Point)),
}

impl Path {
    fn new(start: Point, end: Point) -> Self {
        if start.x == end.x {
            let (start, end) = if start.y <= end.y {
                (start, end)
            } else {
                (end, start)
            };
            Path::Vertical((start.x, start.y, end.y))
        } else if start.y == end.y {
            let (start, end) = if start.x <= end.x {
                (start, end)
            } else {
                (end, start)
            };
            Path::Horizontal((start.y, start.x, end.x))
        } else {
            Path::Diagonal((start, end))
        }
    }
}

#[derive(Debug)]
struct Board {
    grid: [[u8; 1000]; 1000],
}

impl Board {
    pub const fn new() -> Self {
        Self {
            grid: [[0u8; 1000]; 1000],
        }
    }

    pub fn traverse_paths<'a>(&mut self, paths: impl Iterator<Item = &'a Path>) {
        for path in paths {
            match *path {
                Path::Horizontal((y, start_x, end_x)) => {
                    for x in start_x..=end_x {
                        self.grid[usize::from(y)][usize::from(x)] += 1;
                    }
                }
                Path::Vertical((x, start_y, end_y)) => {
                    for y in start_y..=end_y {
                        self.grid[usize::from(y)][usize::from(x)] += 1;
                    }
                }
                Path::Diagonal((start, end)) => {
                    let Point { mut x, mut y } = start;

                    let distance;
                    let x_op = if end.x >= start.x {
                        distance = end.x - start.x;
                        increment
                    } else {
                        distance = start.x - end.x;
                        decrement
                    };

                    let y_op = if end.y >= start.y {
                        increment
                    } else {
                        decrement
                    };

                    for _ in 0..distance {
                        self.grid[usize::from(y)][usize::from(x)] += 1;

                        x_op(&mut x);
                        y_op(&mut y);
                    }

                    self.grid[usize::from(y)][usize::from(x)] += 1;
                }
            }
        }
    }

    pub fn at_least_two_overlap_count(&self) -> usize {
        self.grid
            .iter()
            .flat_map(|row| row.iter())
            .fold(0usize, |count, &item| match item {
                2.. => count + 1,
                _ => count,
            })
    }
}

pub fn part_one(paths: &[Path]) -> usize {
    let mut board = Board::new();

    board.traverse_paths(
        paths
            .iter()
            .filter(|&&path| !matches!(path, Path::Diagonal(_))),
    );
    board.at_least_two_overlap_count()
}

pub fn part_two(paths: &[Path]) -> usize {
    let mut board = Board::new();

    board.traverse_paths(paths.iter());
    board.at_least_two_overlap_count()
}

#[inline]
fn parse_num(index: &mut usize, bytes: &[u8]) -> Num {
    let mut num = Num::from(bytes[*index] - b'0');
    *index += 1;

    for &digit in bytes.iter().skip(*index) {
        if !digit.is_ascii_digit() {
            break;
        }

        num *= 10;
        num += Num::from(digit - b'0');
        *index += 1;
    }
    num
}

pub fn parse_input(input: &str) -> Vec<Path> {
    let mut paths = Vec::with_capacity(500);

    let bytes = input.as_bytes();
    let mut index = 0;

    while index < bytes.len() {
        let x = parse_num(&mut index, bytes);
        index += 1;
        let y = parse_num(&mut index, bytes);

        let start = Point { x, y };
        index += 4;

        let x = parse_num(&mut index, bytes);
        index += 1;
        let y = parse_num(&mut index, bytes);

        let end = Point { x, y };
        index += 1;

        paths.push(Path::new(start, end));
    }

    paths
}

part_impl!(
    part_one_test01: 5,
    part_one_problem: 7468,
    part_two_test01: 12,
    part_two_problem: 22364,
);
