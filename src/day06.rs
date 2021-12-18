mod impl06 {
    #[derive(Clone, Copy, Debug)]
    pub struct Fish {
        days_left: u8,
        created_at: usize,
    }

    impl Fish {
        pub fn init(days_left: u8) -> Self {
            Self {
                days_left,
                created_at: 0,
            }
        }

        pub fn total_spawned(&self, cache: &mut [usize]) -> usize {
            let refreshed = self.refresh();
            let spawn_at = refreshed.created_at;

            if spawn_at >= cache.len() {
                return 0;
            }

            if cache[spawn_at] != 0 {
                return cache[spawn_at];
            }

            let new_fish = Fish::spawn(spawn_at);
            let total = 1 + refreshed.total_spawned(cache) + new_fish.total_spawned(cache);

            cache[spawn_at] = total;
            total
        }

        fn spawn(created_at: usize) -> Self {
            Self {
                days_left: 8,
                created_at,
            }
        }

        fn refresh(&self) -> Self {
            Self {
                days_left: 6,
                created_at: self.created_at + self.days_left as usize + 1,
            }
        }
    }

    #[derive(Debug)]
    pub struct School {
        population: Vec<Fish>,
    }

    impl School {
        pub fn new(population: Vec<Fish>) -> Self {
            Self { population }
        }

        pub fn total_size(&self, num_iters: usize) -> usize {
            let mut cache = vec![0; num_iters + 1];
            self.population
                .iter()
                .map(|fish| 1 + fish.total_spawned(&mut cache))
                .sum()
        }
    }
}

use impl06::*;

pub fn part_one(school: &Input) -> usize {
    school.0.total_size(80)
}

pub fn part_two(school: &Input) -> usize {
    school.0.total_size(256)
}

#[derive(Debug)]
pub struct Input(School);

pub fn parse_input(input: &str) -> Input {
    let mut population = Vec::with_capacity(300);

    let bytes = input.as_bytes();
    let mut index = 0;

    while index < bytes.len() {
        population.push(Fish::init(bytes[index] - b'0'));
        index += 2;
    }

    Input(School::new(population))
}

part_impl! {
    part_one_test01: 5934,
    part_one_problem: 388739,
    part_two_test01: 26984457539,
    part_two_problem: 1741362314973,
}
