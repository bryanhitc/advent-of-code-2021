use advent_of_code_2021::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! benchmark {
    ($day:tt) => {
        |c: &mut Criterion| {
            use $day::*;

            let day_name = stringify!($day);
            let raw_input = std::fs::read_to_string(format!("inputs/{}/problem.txt", day_name)).unwrap();
            let input = parse_input(&raw_input);

            c.bench_function(&format!("{}:Parse", day_name), |b| {
                b.iter(|| {
                    let _test = parse_input(&raw_input);
                })
            });

            c.bench_function(&format!("{}:P1", day_name), |b| {
                b.iter(|| part_one(black_box(&input)))
            });

            c.bench_function(&format!("{}:P2", day_name), |b| {
                b.iter(|| part_two(black_box(&input)))
            });
        }
    };
}

fn day01(c: &mut Criterion) {
    benchmark!(day01)(c)
}

fn day02(c: &mut Criterion) {
    benchmark!(day02)(c)
}

fn day03(c: &mut Criterion) {
    benchmark!(day03)(c)
}

criterion_group!(benches, day01, day02, day03);
criterion_main!(benches);
