use aoc_runner_derive::{aoc, aoc_generator};
use fnv::FnvHashMap;
use itertools::Itertools;

#[aoc_generator(day4)]
fn parse_input_day4(input: &str) -> Result<Vec<FnvHashMap<String, String>>, String> {
    Ok(input
        .split("\n\n")
        .map(|g| {
            g.split_whitespace()
                .flat_map(|kv| kv.split(':').map(|e| e.to_owned()))
                .tuples()
                .collect()
        })
        .collect())
}

fn is_valid(pass: &FnvHashMap<String, String>) -> bool {
    pass.iter().all(|(k, v)| match k.as_ref() {
        "byr" => (1920..=2002).contains(&v.parse().unwrap_or(0)),
        "iyr" => (2010..=2020).contains(&v.parse().unwrap_or(0)),
        "eyr" => (2020..=2030).contains(&v.parse().unwrap_or(0)),
        "pid" => v.len() == 9 && v.chars().all(|c| c.is_ascii_digit()),
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v.as_str()),
        "hcl" => {
            v.starts_with('#') && v.len() == 7 && v.chars().skip(1).all(|c| c.is_ascii_hexdigit())
        }
        "hgt" => {
            let amount = &v[0..v.len() - 2].parse().unwrap_or(0);
            match v[v.len() - 2..].as_ref() {
                "cm" => (150..=193).contains(amount),
                "in" => (59..=76).contains(amount),
                _ => false,
            }
        }
        _ => true,
    })
}

#[aoc(day4, part1)]
fn part1(passports: &[FnvHashMap<String, String>]) -> usize {
    let fields = ["ecl", "eyr", "pid", "hcl", "byr", "iyr", "hgt"];
    passports
        .iter()
        .filter(|p| fields.iter().all(|&k| p.contains_key(k)))
        .count()
}

#[aoc(day4, part2)]
fn part2(passports: &[FnvHashMap<String, String>]) -> usize {
    let fields = ["ecl", "eyr", "pid", "hcl", "byr", "iyr", "hgt"];
    passports
        .iter()
        .filter(|p| fields.iter().all(|&k| p.contains_key(k)) && is_valid(p))
        .count()
}
