use advent_of_code_2021::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// will clean up later via macro or trait
fn day01(c: &mut Criterion) {
    let raw_input = std::fs::read_to_string("inputs/day01/problem.txt").unwrap();
    let input = day01::parse_input(&raw_input);

    c.bench_function("Day01:Parse", |b| {
        b.iter(|| {
            let _test = day01::parse_input(&raw_input);
        })
    });

    c.bench_function("Day01:P1", |b| {
        b.iter(|| day01::part_one(black_box(&input)))
    });

    c.bench_function("Day01:P2", |b| {
        b.iter(|| day01::part_two(black_box(&input)))
    });
}

fn day02(c: &mut Criterion) {
    let raw_input = std::fs::read_to_string("inputs/day02/problem.txt").unwrap();
    let input = day02::parse_input(&raw_input);

    c.bench_function("Day02:Parse", |b| {
        b.iter(|| {
            let _test = day02::parse_input(&raw_input);
        })
    });

    c.bench_function("Day02:P1", |b| {
        b.iter(|| day02::part_one(black_box(&input)))
    });

    c.bench_function("Day02:P2", |b| {
        b.iter(|| day02::part_two(black_box(&input)))
    });
}

fn day03(c: &mut Criterion) {
    let raw_input = std::fs::read_to_string("inputs/day03/problem.txt").unwrap();
    let input = day03::parse_input(&raw_input);

    c.bench_function("Day03:Parse", |b| {
        b.iter(|| {
            let _test = day03::parse_input(&raw_input);
        })
    });

    c.bench_function("Day03:P1", |b| {
        b.iter(|| day03::part_one(black_box(&input)))
    });

    c.bench_function("Day03:P2", |b| {
        b.iter(|| day03::part_two(black_box(&input)))
    });
}

criterion_group!(benches, day01, day02, day03);
criterion_main!(benches);
