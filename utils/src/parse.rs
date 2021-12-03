use std::fmt::Debug;
use std::fs;
use std::path::Path;
use std::str::FromStr;

pub fn iter_input<'a, T, E>(input: &'a str) -> impl Iterator<Item = T> + 'a
where
    T: FromStr<Err = E>,
    E: Debug,
{
    input.lines().map(|line| line.parse::<T>().unwrap())
}

pub fn iter_input_2<'a, T1, T2, E1, E2>(input: &'a str) -> impl Iterator<Item = (T1, T2)> + 'a
where
    T1: FromStr<Err = E1>,
    T2: FromStr<Err = E2>,
    E1: Debug,
    E2: Debug,
{
    input.lines().map(|line| {
        let mut items = line.split(" ");
        (items.next().unwrap().parse::<T1>().unwrap(), items.next().unwrap().parse::<T2>().unwrap())
    })
}

pub fn default_parse_lines<T, E>(input: &str) -> Vec<T>
where
    T: FromStr<Err = E>,
    E: Debug,
{
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<T>>()
}

pub fn parse_lines<T>(input: &str, parser: impl Fn(&str) -> T) -> Vec<T> {
    input.lines().map(parser).collect::<Vec<T>>()
}

pub fn parse_lines2<T1, T2, R>(
    input: &str,
    delimiter: &str,
    parser1: impl Fn(&str) -> T1,
    parser2: impl Fn(&str) -> T2,
    result_builder: impl Fn((T1, T2)) -> R,
) -> Vec<R> {
    input
        .lines()
        .map(|line| line.split(delimiter))
        .map(|mut parts| {
            let one = parser1(parts.next().unwrap());
            let two = parser2(parts.next().unwrap());
            (one, two)
        })
        .map(result_builder)
        .collect::<Vec<R>>()
}

pub fn parse_lines3<T1, T2, T3, R>(
    input: &str,
    delimiter: &str,
    parser1: impl Fn(&str) -> T1,
    parser2: impl Fn(&str) -> T2,
    parser3: impl Fn(&str) -> T3,
    result_builder: impl Fn((T1, T2, T3)) -> R,
) -> Vec<R> {
    input
        .lines()
        .map(|line| line.split(delimiter))
        .map(|mut parts| {
            let one = parser1(parts.next().unwrap());
            let two = parser2(parts.next().unwrap());
            let three = parser3(parts.next().unwrap());
            (one, two, three)
        })
        .map(result_builder)
        .collect::<Vec<R>>()
}

pub fn input_as_string(filename: impl AsRef<Path>) -> String {
    fs::read_to_string(filename).unwrap()
}

pub fn input_as_bytes(filename: impl AsRef<Path>) -> Vec<u8> {
    fs::read(filename).unwrap()
}
