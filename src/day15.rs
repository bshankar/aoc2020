use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug)]
struct GameState {
    seen: [usize; 2021],
    previous: usize,
    current: usize,
    turn: usize,
}

#[aoc_generator(day15)]
fn parse_input_day15(input: &str) -> Result<GameState, String> {
    let mut state = GameState {
        seen: [0; 2021],
        previous: 0,
        current: 0,
        turn: 0,
    };

    input.split(',').enumerate().for_each(|(i, n)| {
        let n = n.parse::<usize>().expect("Invalid integer!");
        state.previous = state.current;
        state.seen[state.previous] = i;
        state.current = n;
        state.turn = i + 1;
    });

    Ok(state)
}

#[aoc(day15, part1)]
fn part1(state: &GameState) -> usize {
    let mut gs = state.clone();
    while gs.turn < 2020 {
        gs.seen[gs.previous] = gs.turn - 1;
        gs.previous = gs.current;
        gs.turn += 1;
        gs.current = if gs.seen[gs.current] == 0 {
            0
        } else {
            gs.turn - gs.seen[gs.current] - 1
        };
    }
    gs.current
}
