use aoc_runner_derive::{aoc, aoc_generator};
use fnv::FnvHashSet;

#[aoc_generator(day17)]
fn parse_input_day17(input: &str) -> Result<FnvHashSet<Vec<i8>>, String> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(j, _)| Ok(vec![i as i8, j as i8]))
        })
        .collect()
}

fn base3(n: i8, dim: i8) -> Vec<i8> {
    let base: i8 = 3;
    (0..dim).map(|d| (n / base.pow(d as u32)) % base).collect()
}

fn origin_neighbors(dim: i8) -> FnvHashSet<Vec<i8>> {
    let base: i8 = 3;
    (1..=base.pow(dim as u32))
        .map(|i| base3(i, dim).iter().map(|c| c - 1).collect())
        .filter(|e: &Vec<_>| e.iter().any(|&v| v != 0))
        .collect()
}

fn get_neighbors(cube: &[i8], origin_neighbors: &FnvHashSet<Vec<i8>>) -> FnvHashSet<Vec<i8>> {
    origin_neighbors
        .iter()
        .map(|n| n.iter().enumerate().map(|(i, c)| c + cube[i]).collect())
        .collect()
}

fn active_neighbors(
    cube: &[i8],
    active: &FnvHashSet<Vec<i8>>,
    origin_neighbors: &FnvHashSet<Vec<i8>>,
) -> usize {
    get_neighbors(cube, origin_neighbors)
        .intersection(active)
        .count()
}

fn cycle(
    active: &FnvHashSet<Vec<i8>>,
    origin_neighbors: &FnvHashSet<Vec<i8>>,
) -> FnvHashSet<Vec<i8>> {
    let mut to_remove = FnvHashSet::default();
    let mut to_add = FnvHashSet::default();

    for cube in active.iter() {
        if ![2, 3].contains(&active_neighbors(cube, active, origin_neighbors)) {
            to_remove.insert(cube);
        }

        to_add.extend(
            get_neighbors(cube, origin_neighbors)
                .difference(active)
                .filter(|c| active_neighbors(c, active, origin_neighbors) == 3)
                .cloned(),
        )
    }

    active
        .iter()
        .chain(&to_add)
        .filter(|e| !to_remove.contains(e))
        .cloned()
        .collect()
}

#[aoc(day17, part1)]
fn part1(active: &FnvHashSet<Vec<i8>>) -> usize {
    let mut a = active.iter().map(|e| vec![e[0], e[1], 0]).collect();
    let on = origin_neighbors(3);
    (0..6).for_each(|_| a = cycle(&a, &on));
    a.len()
}

#[aoc(day17, part2)]
fn part2(active: &FnvHashSet<Vec<i8>>) -> usize {
    let mut a = active.iter().map(|e| vec![e[0], e[1], 0, 0]).collect();
    let on = origin_neighbors(4);
    (0..6).for_each(|_| a = cycle(&a, &on));
    a.len()
}
