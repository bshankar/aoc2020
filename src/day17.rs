use aoc_runner_derive::{aoc, aoc_generator};
use fnv::FnvHashSet;

type Point = (i8, i8, i8);

#[aoc_generator(day17)]
fn parse_input_day17(input: &str) -> Result<FnvHashSet<Point>, String> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(j, _)| Ok((i as i8, j as i8, 0)))
        })
        .collect()
}

fn base3(n: i8) -> Point {
    (n % 3, (n / 3) % 3, (n / 9) % 3)
}

fn get_neighbors(cube: &Point) -> FnvHashSet<Point> {
    (1..=27)
        .map(|i| {
            let (x, y, z) = base3(i);
            (cube.0 + x - 1, cube.1 + y - 1, cube.2 + z - 1)
        })
        .filter(|e| e != cube)
        .collect()
}

fn active_neighbors(cube: &Point, active: &FnvHashSet<Point>) -> usize {
    get_neighbors(cube)
        .iter()
        .filter(|c| active.contains(c))
        .count()
}

fn cycle(active: &FnvHashSet<Point>) -> FnvHashSet<Point> {
    let mut to_remove = Vec::new();
    let mut to_add = Vec::new();

    for cube in active.iter() {
        if ![2, 3].contains(&active_neighbors(cube, active)) {
            to_remove.push(cube);
        }

        to_add.extend(get_neighbors(cube).iter().filter(|c| {
            !active.contains(c)
                && get_neighbors(c)
                    .iter()
                    .filter(|n| active.contains(n))
                    .count()
                    == 3
        }))
    }

    active
        .iter()
        .chain(&to_add)
        .filter(|e| !to_remove.contains(e))
        .copied()
        .collect()
}

#[aoc(day17, part1)]
fn part1(active: &FnvHashSet<Point>) -> usize {
    let mut a = cycle(active);
    (0..5).for_each(|_| a = cycle(&a));
    a.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbors_are_26() {
        assert_eq!(get_neighbors(&(0, 0, 0)).len(), 26);
    }
}
