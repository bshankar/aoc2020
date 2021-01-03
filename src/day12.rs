use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day12)]
fn parse_input_day11(input: &str) -> Result<Vec<String>, String> {
    Ok(input.lines().map(|l| l.to_owned()).collect())
}

fn rotate_left(facing: &mut [isize; 2], amount: isize) {
    match amount {
        90 => {
            let tmp = facing[0];
            facing[0] = -facing[1];
            facing[1] = tmp;
        }
        180 => {
            facing[0] = -facing[0];
            facing[1] = -facing[1];
        }
        270 => {
            let tmp = facing[0];
            facing[0] = facing[1];
            facing[1] = -tmp;
        }
        _ => panic!("Unknown amount of rotation"),
    };
}

#[aoc(day12, part1)]
fn part1(walks: &[String]) -> isize {
    let mut current = [0, 0];
    let mut facing = [1, 0];

    for walk in walks {
        let (direction, amount) = walk.split_at(1);
        let amount = amount.parse().unwrap_or(0);

        match direction {
            "N" => current[1] += amount,
            "S" => current[1] -= amount,
            "E" => current[0] += amount,
            "W" => current[0] -= amount,
            "F" => {
                current[0] += facing[0] * amount;
                current[1] += facing[1] * amount
            }
            "L" => rotate_left(&mut facing, amount),
            "R" => rotate_left(&mut facing, 360 - amount),
            _ => panic!("Unknown direction!"),
        }
    }
    current.iter().map(|v| v.abs()).sum()
}

#[aoc(day12, part2)]
fn part2(walks: &[String]) -> isize {
    let mut current = [0, 0];
    let mut facing = [10, 1];

    for walk in walks {
        let (direction, amount) = walk.split_at(1);
        let amount = amount.parse().unwrap_or(0);

        match direction {
            "N" => facing[1] += amount,
            "S" => facing[1] -= amount,
            "E" => facing[0] += amount,
            "W" => facing[0] -= amount,
            "F" => {
                current[0] += facing[0] * amount;
                current[1] += facing[1] * amount
            }
            "L" => rotate_left(&mut facing, amount),
            "R" => rotate_left(&mut facing, 360 - amount),
            _ => panic!("Unknown direction!"),
        }
    }
    current.iter().map(|v| v.abs()).sum()
}
