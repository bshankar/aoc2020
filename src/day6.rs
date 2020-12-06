use aoc_runner_derive::{aoc, aoc_generator};
use fnv::FnvHashSet;

#[aoc_generator(day6)]
fn parse_input_day6(input: &str) -> Result<Vec<Vec<FnvHashSet<char>>>, String> {
    let mut answers = Vec::new();
    let mut current = Vec::new();

    for line in input.lines() {
        if line == "" {
            answers.push(current);
            current = Vec::new();
        } else {
            let ans = line
                .chars()
                .filter(|c| c.is_ascii_alphabetic() && c.is_lowercase())
                .collect::<FnvHashSet<_>>();
            current.push(ans);
        }
    }
    answers.push(current);
    Ok(answers)
}

#[aoc(day6, part1)]
fn part1(answers: &[Vec<FnvHashSet<char>>]) -> usize {
    answers
        .iter()
        .map(|ans| {
            ans.iter()
                .fold(FnvHashSet::default(), |acc, x| {
                    acc.union(x).copied().collect()
                })
                .len()
        })
        .sum()
}

#[aoc(day6, part2)]
fn part2(answers: &[Vec<FnvHashSet<char>>]) -> usize {
    answers
        .iter()
        .map(|ans| {
            ans.iter()
                .fold(ans[0].clone(), |acc, x| {
                    acc.intersection(x).copied().collect()
                })
                .len()
        })
        .sum()
}
