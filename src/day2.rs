use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};
use std::error::Error;

#[derive(Display, FromStr)]
#[display("{min}-{max} {value}: {password}")]
struct Policy {
    min: usize,
    max: usize,
    value: char,
    password: String,
}

#[aoc_generator(day2)]
fn parse_input_day2(input: &str) -> Result<Vec<Policy>, impl Error> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day2, part1)]
fn part1(policies: &[Policy]) -> usize {
    policies
        .iter()
        .filter(|p| {
            let count = p.password.chars().filter(|&c| c == p.value).count();
            count >= p.min && count <= p.max
        })
        .count()
}

#[aoc(day2, part2)]
fn part2(policies: &[Policy]) -> usize {
    policies
        .iter()
        .filter(|p| {
            let chars: Vec<char> = p.password.chars().collect();
            (chars[p.min - 1] == p.value) ^ (chars[p.max - 1] == p.value)
        })
        .count()
}
