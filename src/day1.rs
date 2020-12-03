use aoc_runner_derive::{aoc, aoc_generator};
use fnv::FnvHashSet;
use std::iter::FromIterator;
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<Vec<usize>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day1, part1)]
fn part1(nos: &[usize]) -> Option<usize> {
    let seen = FnvHashSet::from_iter(nos);

    nos.iter()
        .find(|&&i| seen.contains(&(2020 - i)))
        .map(|&i| i * (2020 - i))
}

#[aoc(day1, part2)]
fn part2(nos: &[usize]) -> Option<usize> {
    let seen = FnvHashSet::from_iter(nos);

    nos.iter()
        .filter_map(|i| {
            nos.iter()
                .find(|&j| i + j < 2020 && seen.contains(&(2020 - i - j)))
                .map(|j| i * j * (2020 - i - j))
        })
        .next()
}
