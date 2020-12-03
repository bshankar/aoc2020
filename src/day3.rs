use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse_input_day3(input: &str) -> Result<Vec<Vec<bool>>, String> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => Ok(true),
                    '.' => Ok(false),
                    _ => Err("Invalid char".to_owned()),
                })
                .collect()
        })
        .collect()
}

fn walk_trees(trees: &[Vec<bool>], x_inc: usize, y_inc: usize) -> usize {
    (0..trees.len())
        .step_by(y_inc)
        .enumerate()
        .map(|(i, y)| ((i * x_inc) % trees[y].len(), y))
        .filter(|&(x, y)| trees[y][x])
        .count()
}

#[aoc(day3, part1)]
fn part1(trees: &[Vec<bool>]) -> usize {
    walk_trees(trees, 3, 1)
}

#[aoc(day3, part2)]
fn part2(trees: &[Vec<bool>]) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(x, y)| walk_trees(trees, x, y))
        .product()
}
