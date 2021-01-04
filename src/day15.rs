use aoc_runner_derive::{aoc, aoc_generator};
use fnv::FnvHashMap;

#[derive(Clone, Debug)]
struct GameState {
    seen: FnvHashMap<usize, usize>,
    previous: usize,
    current: usize,
    turn: usize,
}

#[aoc_generator(day15)]
fn parse_input_day15(input: &str) -> Result<GameState, String> {
    let mut state = GameState {
        seen: FnvHashMap::default(),
        previous: 0,
        current: 0,
        turn: 0,
    };

    input.split(',').enumerate().for_each(|(i, n)| {
        let n = n.parse::<usize>().expect("Invalid integer!");
        state.previous = state.current;
        state.seen.insert(state.previous, i);
        state.current = n;
        state.turn = i + 1;
    });

    Ok(state)
}

fn play(state: &GameState, iterations: usize) -> usize {
    let mut gs = state.clone();
    while gs.turn < iterations {
        gs.seen.insert(gs.previous, gs.turn - 1);
        gs.previous = gs.current;
        gs.turn += 1;
        gs.current = if !gs.seen.contains_key(&gs.current) {
            0
        } else {
            gs.turn - gs.seen[&gs.current] - 1
        };
    }
    gs.current
}

#[aoc(day15, part1)]
fn part1(state: &GameState) -> usize {
    play(state, 2020)
}

#[aoc(day15, part2)]
fn part2(state: &GameState) -> usize {
    play(state, 30_000_000)
}
