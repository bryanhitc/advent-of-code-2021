use advent_of_code_2021::*;
use criterion::{Criterion, black_box, criterion_group, criterion_main};

macro_rules! benchmark {
    ($day_name:expr, $module:tt) => {
        |c: &mut Criterion| {
            use $module::{parse_input, part_one, part_two};

            let day_name = $day_name;
            let module_name = stringify!($module);

            let full_prefix = format!("{}::{}", day_name, module_name);
            let prefix = if day_name == module_name {
                day_name
            } else {
                full_prefix.as_str()
            };

            let raw_input =
                std::fs::read_to_string(format!("inputs/{}/problem.txt", day_name)).unwrap();
            let input = parse_input(&raw_input);

            c.bench_function(&format!("{}::parse", prefix), |b| {
                b.iter(|| {
                    let _test = parse_input(&raw_input);
                })
            });

            c.bench_function(&format!("{}::part1", prefix), |b| {
                b.iter(|| part_one(black_box(&input)))
            });

            c.bench_function(&format!("{}::part2", prefix), |b| {
                b.iter(|| part_two(black_box(&input)))
            });
        }
    };
}

fn day01(c: &mut Criterion) {
    benchmark!("day01", day01)(c)
}

fn day02(c: &mut Criterion) {
    benchmark!("day02", day02)(c)
}

fn day03(c: &mut Criterion) {
    benchmark!("day03", day03)(c)
}

fn day04(c: &mut Criterion) {
    benchmark!("day04", day04)(c)
}

fn day05(c: &mut Criterion) {
    benchmark!("day05", day05)(c)
}

fn day06(c: &mut Criterion) {
    benchmark!("day06", day06)(c)
}

fn day07(c: &mut Criterion) {
    use day07::impl07;
    benchmark!("day07", impl07)(c)
}

fn day07_opt(c: &mut Criterion) {
    use day07::implopt07;
    benchmark!("day07", implopt07)(c)
}

fn day09(c: &mut Criterion) {
    use day09::impl09;
    benchmark!("day09", impl09)(c)
}

fn day09_opt(c: &mut Criterion) {
    use day09::implopt09;
    benchmark!("day09", implopt09)(c)
}

fn day10(c: &mut Criterion) {
    benchmark!("day10", day10)(c)
}

criterion_group!(
    benches, day01, day02, day03, day04, day05, day06, day07, day07_opt, day09, day09_opt, day10
);
criterion_main!(benches);
