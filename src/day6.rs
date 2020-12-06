use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn parse_input_day6(input: &str) -> Result<Vec<Vec<u32>>, String> {
    Ok(input
        .split("\n\n")
        .map(|g| {
            g.lines()
                .map(|a| {
                    a.bytes()
                        .filter(|c| c.is_ascii_alphabetic())
                        .fold(0, |acc, c| acc | 1 << (c - 97))
                })
                .collect()
        })
        .collect())
}

#[aoc(day6, part1)]
fn part1(answers: &[Vec<u32>]) -> u32 {
    answers
        .iter()
        .map(|ans| ans.iter().fold(0, |acc, x| acc | x).count_ones())
        .sum()
}

#[aoc(day6, part2)]
fn part2(answers: &[Vec<u32>]) -> u32 {
    answers
        .iter()
        .map(|ans| ans.iter().fold(u32::MAX, |acc, x| acc & x).count_ones())
        .sum()
}
