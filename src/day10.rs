use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day10)]
fn parse_input_day10(input: &str) -> Result<Vec<usize>, ParseIntError> {
    let mut adapters = input
        .lines()
        .map(|n| n.parse().unwrap_or(0))
        .collect::<Vec<_>>();

    adapters.sort_unstable();
    Ok(adapters)
}

#[aoc(day10, part1)]
fn part1(adapters: &[usize]) -> usize {
    let adapters = adapters.to_vec();
    let result = adapters.iter().fold((0, 0, 0), |(one, three, prev), &v| {
        let delta = v - prev;
        if delta == 1 {
            (one + 1, three, v)
        } else if delta == 3 {
            (one, three + 1, v)
        } else {
            (one, three, prev)
        }
    });

    result.0 * (result.1 + 1)
}

#[aoc(day10, part2)]
fn part2(adapters: &[usize]) -> usize {
    let max = adapters.iter().max().unwrap_or(&0).to_owned();
    let mut count = vec![0; max + 1];

    adapters
        .iter()
        .filter(|&&a| a <= 3)
        .for_each(|&a| count[a] = 1);

    adapters.iter().for_each(|a| {
        (1..=3)
            .filter(|i| a > i && adapters.contains(&(a - i)))
            .for_each(|i| count[*a] += count[a - i])
    });
    count[max]
}
