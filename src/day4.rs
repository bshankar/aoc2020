use aoc_runner_derive::{aoc, aoc_generator};
use fnv::FnvHashMap;

#[aoc_generator(day4)]
fn parse_input_day3(input: &str) -> Result<Vec<FnvHashMap<String, String>>, String> {
    let mut passports = Vec::new();
    let mut current = FnvHashMap::default();

    for line in input.lines() {
        if line == "" {
            passports.push(current);
            current = FnvHashMap::default();
        } else {
            line.split_whitespace().for_each(|kv| {
                let res: Vec<_> = kv.split(':').collect();
                current.insert(res[0].to_owned(), res[1].to_owned());
            })
        }
    }
    passports.push(current);
    Ok(passports)
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
