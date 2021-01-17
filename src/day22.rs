use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::VecDeque;
use std::num::ParseIntError;

type Player = VecDeque<usize>;
type PlayersState = Vec<Player>;

#[aoc_generator(day22)]
fn parse_input_day1(input: &str) -> Result<PlayersState, ParseIntError> {
    input
        .split("\n\n")
        .map(|p| {
            p.lines()
                .filter(|l| !l.starts_with("Player"))
                .map(|l| l.parse())
                .collect()
        })
        .collect()
}

fn play_turn(players: &mut [Player], winner: Option<usize>) -> usize {
    let card1 = players[0].pop_front().unwrap();
    let card2 = players[1].pop_front().unwrap();
    let winner = winner.unwrap_or_else(|| if card1 > card2 { 0 } else { 1 });

    if winner == 0 {
        players[0].push_back(card1);
        players[0].push_back(card2);
    } else {
        players[1].push_back(card2);
        players[1].push_back(card1);
    }
    winner
}

fn play_game(players: &mut [Player]) -> usize {
    let mut winner = 0;
    while !players.iter().any(|p| p.is_empty()) {
        winner = play_turn(players, None);
    }
    winner
}

fn compute_score(players: &[Player], winner: usize) -> usize {
    players[winner]
        .iter()
        .enumerate()
        .fold(0, |acc, (i, c)| acc + (players[winner].len() - i) * c)
}

#[aoc(day22, part1)]
fn part1(players: &[Player]) -> usize {
    let mut tmp = players.to_vec();
    let winner = play_game(&mut tmp);
    compute_score(&tmp, winner)
}

fn play_recursive(players: &mut [Player], seen: &mut Vec<PlayersState>, winner: &mut usize) {
    while !players.iter().any(|p| p.is_empty()) {
        if seen.contains(&players.to_vec()) {
            *winner = 0;
            return;
        } else {
            seen.push(players.to_vec());
            let card1 = players[0][0];
            let card2 = players[1][0];
            if players[0].len() > card1 && players[1].len() > card2 {
                play_recursive(
                    vec![
                        players[0].iter().skip(1).take(card1).copied().collect(),
                        players[1].iter().skip(1).take(card2).copied().collect(),
                    ]
                    .as_mut(),
                    &mut Vec::new(),
                    winner,
                );
                play_turn(players, Some(*winner));
            } else {
                *winner = play_turn(players, None);
            }
        }
    }
}

#[aoc(day22, part2)]
fn part2(players: &[Player]) -> usize {
    let mut tmp = players.to_vec();
    let mut winner = 0;
    play_recursive(&mut tmp, &mut Vec::new(), &mut winner);
    compute_score(&tmp, winner)
}
