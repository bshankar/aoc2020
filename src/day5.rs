use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn parse_input_day5(input: &str) -> Result<Vec<String>, String> {
    Ok(input.lines().map(|s| s.to_owned()).collect())
}

fn bisect(min: usize, max: usize, code: &str, index: usize) -> usize {
    if min == max {
        min
    } else {
        match code.chars().nth(index).unwrap_or('0') {
            'F' | 'L' => bisect(min, (min + max) / 2, code, index + 1),
            'R' | 'B' => bisect((min + max) / 2 + 1, max, code, index + 1),
            _ => 0,
        }
    }
}

#[aoc(day5, part1)]
fn part1(codes: &[String]) -> Option<usize> {
    codes
        .iter()
        .map(|s| bisect(0, 127, &s[..7], 0) * 8 + bisect(0, 7, &s[7..], 0))
        .max()
}

#[aoc(day5, part2)]
fn part2(codes: &[String]) -> Option<usize> {
    let seats = codes
        .iter()
        .map(|s| bisect(0, 127, &s[..7], 0) * 8 + bisect(0, 7, &s[7..], 0))
        .collect::<Vec<_>>();

    (*seats.iter().min().unwrap_or(&0)..*seats.iter().max().unwrap_or(&1000))
        .find(|s| !seats.contains(s))
}
