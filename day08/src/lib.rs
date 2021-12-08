use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
enum Part {
    One,
    Two,
}

#[allow(dead_code)]
fn do_it(part: Part, content: &str) -> usize {
    let lines: Vec<Vec<_>> = content.lines().map(|l| l.split(" | ").collect()).collect();

    let mut inputs: Vec<(Vec<&str>, Vec<&str>)> = Vec::new();
    for line in &lines {
        let input_parts: Vec<_> = line[0].split_whitespace().collect();
        let output_parts: Vec<_> = line[1].split_whitespace().collect();
        inputs.push((input_parts, output_parts));
    }

    let outputs: Vec<Vec<_>> = lines
        .iter()
        .map(|l| l[1].split_whitespace().collect())
        .collect();

    match part {
        Part::One => {
            let mut tally = 0;
            for output in outputs {
                for digit in output {
                    tally += match digit.len() {
                            2 => 1,
                            3 => 1,
                            4 => 1,
                            7 => 1,
                            _ => 0,
                        };
                }
            }
            tally
        }
        Part::Two => {
            let mut sum = 0;
            for (src, dst) in inputs {
                let mut digits: HashMap<String, &str> = HashMap::new();
                let mut five_mers: HashSet<_> = HashSet::new();
                let mut six_mers: HashSet<_> = HashSet::new();

                for combo in &src {
                    match combo.len() {
                        2 => {
                            digits.insert("1".to_string(), combo);
                        }
                        3 => {
                            digits.insert("7".to_string(), combo);
                        }
                        4 => {
                            digits.insert("4".to_string(), combo);
                        }
                        5 => {
                            five_mers.insert(combo);
                        }
                        6 => {
                            six_mers.insert(combo);
                        }
                        7 => {
                            digits.insert("8".to_string(), combo);
                        }
                        _ => (),
                    };
                }

                let one_set: HashSet<char> = digits["1"].chars().collect();
                let four_set: HashSet<char> = digits["4"].chars().collect();
                for combo in six_mers.drain() {
                    let six_mer_set: HashSet<char> = combo.chars().collect();
                    if six_mer_set
                        .intersection(&one_set)
                        .collect::<HashSet<_>>()
                        .len()
                        == 1
                    {
                        digits.insert("6".to_string(), combo);
                    } else if six_mer_set
                        .difference(&four_set)
                        .collect::<HashSet<_>>()
                        .len()
                        == 2
                    {
                        digits.insert("9".to_string(), combo);
                    } else {
                        digits.insert("0".to_string(), combo);
                    }
                }

                let seven_set: HashSet<char> = digits["7"].chars().collect();
                let six_set: HashSet<char> = digits["6"].chars().collect();
                for combo in five_mers.drain() {
                    let five_mer_set: HashSet<char> = combo.chars().collect();
                    if five_mer_set
                        .difference(&seven_set)
                        .collect::<HashSet<_>>()
                        .len()
                        == 2
                    {
                        digits.insert("3".to_string(), combo);
                    } else if six_set
                        .difference(&five_mer_set)
                        .collect::<HashSet<_>>()
                        .len()
                        == 1
                    {
                        digits.insert("5".to_string(), combo);
                    } else {
                        digits.insert("2".to_string(), combo);
                    }
                }

                let mut inverted: HashMap<String, String> = HashMap::new();
                for (k, v) in digits.iter() {
                    let sorted_chars = sort_string(v.to_string());
                    inverted.insert(sorted_chars, k.to_string());
                }

                let output = format!(
                    "{}{}{}{}",
                    inverted[&sort_string(dst[0].to_string())],
                    inverted[&sort_string(dst[1].to_string())],
                    inverted[&sort_string(dst[2].to_string())],
                    inverted[&sort_string(dst[3].to_string())]
                );

                let parsed: usize = output.parse().unwrap();
                sum += parsed;
            }

            sum
        }
    }
}

fn sort_string(v: String) -> String {
    let mut chars: Vec<char> = v.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    String::from_iter(chars)
}

#[cfg(test)] // cargo test -- --show-output [TEST_NAME]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("data/sample.txt");
    const INPUT: &str = include_str!("data/input.txt");

    #[test]
    fn part_1() {
        assert_eq!(do_it(Part::One, SAMPLE), 26);
        assert_eq!(do_it(Part::One, INPUT), 479);
    }

    #[test]
    fn part_2() {
        assert_eq!(do_it(Part::Two, SAMPLE), 61229);
        assert_eq!(do_it(Part::Two, INPUT), 1041746);
    }
}
