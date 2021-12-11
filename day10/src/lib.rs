use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
enum Part {
    One,
    Two,
}

type Instruction = Vec<char>;

#[allow(dead_code)]
fn do_it(part: Part, content: &str) -> usize {
    let mut instructions: Vec<Instruction> = content
        .lines()
        .map(|l| close_pairs(&mut l.chars().collect::<Vec<_>>()))
        .collect();

    match part {
        Part::One => instructions.iter().map(|i| first_mismatch_score(i)).sum(),
        Part::Two => {
            instructions = instructions
                .into_iter()
                .filter(|i| first_mismatch_score(i) == 0)
                .collect();
            let mut scores: Vec<usize> = instructions
                .iter()
                .map(|i| {
                    let mut score = 0;
                    for c in i.iter().rev() {
                        score *= 5;
                        score += match c {
                            '(' => 1,
                            '[' => 2,
                            '{' => 3,
                            '<' => 4,
                            _ => 0,
                        };
                    }
                    score
                })
                .collect();

            scores.sort_unstable();

            scores[(scores.len() as f32 / 2.0).floor() as usize]
        }
    }
}

fn first_mismatch_score(instruction: &[char]) -> usize {
    let closing: HashSet<char> = HashSet::from(['}', '>', ']', ')']);
    let pair: HashMap<char, char> = HashMap::from([('}', '{'), ('>', '<'), (']', '['), (')', '(')]);

    for i in 1..instruction.len() {
        if closing.contains(&instruction[i]) && pair[&instruction[i]] != instruction[i - 1] {
            return match instruction[i] {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0,
            };
        }
    }

    0
}

fn close_pairs(instruction: &mut Instruction) -> Instruction {
    let pair: HashMap<char, char> = HashMap::from([
        ('{', '}'),
        ('<', '>'),
        ('[', ']'),
        ('(', ')'),
        ('}', '*'),
        ('>', '*'),
        (']', '*'),
        (')', '*'),
    ]);

    for i in 0..instruction.len() - 1 {
        if instruction[i + 1] == pair[&instruction[i]] {
            instruction.remove(i + 1);
            instruction.remove(i);
            return close_pairs(instruction);
        }
    }

    instruction.to_vec()
}

#[cfg(test)] // cargo test -- --show-output [TEST_NAME]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("data/sample.txt");
    const INPUT: &str = include_str!("data/input.txt");

    #[test]
    fn part_1() {
        assert_eq!(do_it(Part::One, SAMPLE), 26397);
        assert_eq!(do_it(Part::One, INPUT), 167379);
    }

    #[test]
    fn part_2() {
        assert_eq!(do_it(Part::Two, SAMPLE), 288957);
        assert_eq!(do_it(Part::Two, INPUT), 2776842859);
    }
}
