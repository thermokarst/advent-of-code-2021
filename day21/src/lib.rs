use std::cmp::{max, min};
use std::collections::HashMap;

#[allow(dead_code)]
enum Part {
    One,
    Two,
}

type StartingPositions = (usize, usize);

enum Player {
    One,
    Two,
}

fn move_pawn(mut pos: usize, steps: usize) -> usize {
    for _step in 0..steps {
        pos += 1;
        if pos > 10 {
            pos = 1;
        }
    }
    pos
}

type State = HashMap<(usize, usize, usize, usize), (usize, usize)>;

struct DeterministicDie {
    value: usize,
    roll_count: usize,
}

impl DeterministicDie {
    fn roll(&mut self) -> usize {
        let rolls = self.value * 3 + 3;
        self.value += 3;
        self.roll_count += 3;
        rolls
    }
}

#[allow(dead_code)]
fn do_it(part: Part, positions: StartingPositions) -> usize {
    let mut p1_score = 0;
    let mut p2_score = 0;
    let (mut p1_pos, mut p2_pos) = positions;
    let mut die = DeterministicDie {
        value: 1,
        roll_count: 0,
    };
    let mut turn = Player::One;
    let mut max_score = 0;

    match part {
        Part::One => {
            while max_score < 1000 {
                let sum = die.roll();

                turn = match turn {
                    Player::One => {
                        p1_pos = move_pawn(p1_pos, sum);
                        p1_score += p1_pos;
                        Player::Two
                    }

                    Player::Two => {
                        p2_pos = move_pawn(p2_pos, sum);
                        p2_score += p2_pos;
                        Player::One
                    }
                };

                max_score = max(p1_score, p2_score);
            }

            min(p1_score, p2_score) * die.roll_count
        }
        Part::Two => {
            let mut rolls: Vec<usize> = Vec::new();
            for d1 in 1..4 {
                for d2 in 1..4 {
                    for d3 in 1..4 {
                        rolls.push(d1 + d2 + d3);
                    }
                }
            }

            let mut state: State = State::new();
            let games = quantum_game(&rolls, &mut state, p1_pos, p2_pos, 0, 0);

            max(games.0, games.1)
        }
    }
}

fn quantum_game(
    rolls: &[usize],
    state: &mut State,
    p1_pos: usize,
    p2_pos: usize,
    p1_score: usize,
    p2_score: usize,
) -> (usize, usize) {
    if p2_score >= 21 {
        return (0, 1);
    }

    let snapshot = (p1_pos, p2_pos, p1_score, p2_score);

    if let Some(&score) = state.get(&snapshot) {
        return score;
    }

    let mut games = (0, 0);

    for roll in rolls {
        let p1_pos_new = move_pawn(p1_pos, *roll);
        let p1_score_new = p1_score + p1_pos_new;
        let (a, b) = quantum_game(rolls, state, p2_pos, p1_pos_new, p2_score, p1_score_new);
        games = (games.0 + b, games.1 + a);
    }

    state.insert(snapshot, games);

    games
}

#[cfg(test)] // cargo test -- --show-output [TEST_NAME]
mod tests {
    use super::*;

    const SAMPLE: StartingPositions = (4, 8);
    const INPUT: StartingPositions = (10, 2);

    #[test]
    fn part_1() {
        assert_eq!(do_it(Part::One, SAMPLE), 739785);
        assert_eq!(do_it(Part::One, INPUT), 916083);
    }

    #[test]
    fn part_2() {
        assert_eq!(do_it(Part::Two, SAMPLE), 444356092776315);
        assert_eq!(do_it(Part::Two, INPUT), 49982165861983);
    }
}
