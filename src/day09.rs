type Height = u8;
type Answer = u32;

pub mod impl09 {
    use super::*;
    use std::collections::BTreeSet;

    #[derive(Debug)]
    pub struct HeightMap {
        grid: Vec<Vec<Height>>,
    }

    impl HeightMap {
        fn get_basin_size(
            &self,
            visited: &mut BTreeSet<(usize, usize)>,
            x: usize,
            y: usize,
        ) -> u32 {
            if self.grid[y][x] == 9 || visited.contains(&(x, y)) {
                return 0;
            }

            let mut result = 1;
            visited.insert((x, y));

            if x > 0 {
                result += self.get_basin_size(visited, x - 1, y);
            }

            if x < self.grid[y].len() - 1 {
                result += self.get_basin_size(visited, x + 1, y);
            }

            if y > 0 {
                result += self.get_basin_size(visited, x, y - 1);
            }

            if y < self.grid.len() - 1 {
                result += self.get_basin_size(visited, x, y + 1);
            }

            result
        }

        fn is_lower_point(&self, x: usize, y: usize) -> bool {
            let row = &self.grid[y];
            let height = row[x];

            if x > 0 && height >= row[x - 1] {
                return false;
            }

            if x < row.len() - 1 && height >= row[x + 1] {
                return false;
            }

            if y > 0 && height >= self.grid[y - 1][x] {
                return false;
            }

            if y < self.grid.len() - 1 && height >= self.grid[y + 1][x] {
                return false;
            }

            true
        }

        pub fn part_one(&self) -> Answer {
            let mut result = 0;

            for y in 0..self.grid.len() {
                for x in 0..self.grid[y].len() {
                    if self.is_lower_point(x, y) {
                        result += Answer::from(self.grid[y][x]) + 1;
                    }
                }
            }

            result
        }

        pub fn part_two(&self) -> Answer {
            // todo: use heap and pop 3 times
            let mut basins = Vec::new();
            let mut visited = BTreeSet::new();

            for y in 0..self.grid.len() {
                for x in 0..self.grid[y].len() {
                    if self.is_lower_point(x, y) {
                        basins.push(self.get_basin_size(&mut visited, x, y));
                        visited.clear();
                    }
                }
            }

            basins.sort_unstable();
            basins.iter().rev().take(3).product()
        }
    }

    pub fn part_one(height_map: &HeightMap) -> Answer {
        height_map.part_one()
    }

    pub fn part_two(height_map: &HeightMap) -> Answer {
        height_map.part_two()
    }

    pub fn parse_input(input: &str) -> HeightMap {
        let mut grid = Vec::new();
        let mut row = Vec::new();

        for &byte in input.as_bytes() {
            if let b'\n' = byte {
                grid.push(row.to_vec());
                row.clear();
            } else {
                row.push(byte - b'0');
            }
        }

        HeightMap { grid }
    }
}

pub mod implopt09 {
    use super::*;
    use std::collections::BinaryHeap;

    const VISITED: Height = b'9';

    #[derive(Clone, Debug)]
    pub struct HeightMap {
        grid: Vec<Height>,
        row_len: usize,
    }

    impl HeightMap {
        fn get_basin_size(&mut self, index: usize, col_index: usize) -> u32 {
            if self.grid[index] == VISITED {
                return 0;
            }

            let mut result = 1;
            self.grid[index] = VISITED;

            if col_index > 0 {
                result += self.get_basin_size(index - 1, col_index - 1);
            }

            if col_index + 1 < self.row_len {
                result += self.get_basin_size(index + 1, col_index + 1);
            }

            if index >= self.row_len {
                result += self.get_basin_size(index - self.row_len, col_index);
            }

            if index + self.row_len < self.grid.len() {
                result += self.get_basin_size(index + self.row_len, col_index);
            }

            result
        }

        // should be able to make this much faster by checking the
        // inner square separately without the bounds checking.
        fn is_lower_point(&self, index: usize, col_index: usize) -> bool {
            // can mark surrounding points that have a greater height
            // as NOT a lower point to prevent duplicate checking?
            let height = self.grid[index];

            if col_index > 0 && height >= self.grid[index - 1] {
                return false;
            }

            if col_index + 1 < self.row_len && height >= self.grid[index + 1] {
                return false;
            }

            if index >= self.row_len && height >= self.grid[index - self.row_len] {
                return false;
            }

            if index + self.row_len < self.grid.len() && height >= self.grid[index + self.row_len] {
                return false;
            }

            true
        }

        pub fn part_one(&self) -> Answer {
            let mut result = 0;
            let mut index = 0;

            while index < self.grid.len() {
                for col_index in 0..self.row_len {
                    if self.is_lower_point(index, col_index) {
                        result += Answer::from(self.grid[index] - b'0') + 1;
                    }

                    index += 1;
                }
            }

            result
        }

        pub fn part_two(&mut self) -> Answer {
            let mut basins = BinaryHeap::new();
            let mut index = 0;

            while index < self.grid.len() {
                for col_index in 0..self.row_len {
                    if self.is_lower_point(index, col_index) {
                        basins.push(self.get_basin_size(index, col_index));
                    }

                    index += 1;
                }
            }

            basins.pop().unwrap() * basins.pop().unwrap() * basins.pop().unwrap()
        }
    }

    pub fn part_one(height_map: &HeightMap) -> Answer {
        height_map.part_one()
    }

    pub fn part_two(height_map: &HeightMap) -> Answer {
        height_map.clone().part_two()
    }

    pub fn parse_input(input: &str) -> HeightMap {
        let bytes = input.as_bytes();
        let mut grid = Vec::with_capacity(bytes.len());
        let mut index = 0;

        while bytes[index] != b'\n' {
            grid.push(bytes[index]);
            index += 1;
        }

        let row_len = index;

        // skip final newline
        while index < bytes.len() - 1 {
            index += 1; // skip newline

            for _ in 0..row_len {
                grid.push(bytes[index]);
                index += 1;
            }
        }

        HeightMap { grid, row_len }
    }
}

use implopt09::*;

part_impl! {
    part_one_test01: 15,
    part_one_problem: 631,
    part_two_test01: 1134,
    part_two_problem: 821560,
}
