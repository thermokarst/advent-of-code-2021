use std::cmp::max;
use std::collections::HashMap;

#[allow(dead_code)]
enum Part {
    One,
    Two,
}

type Polymer = Vec<String>;
type Lookup = HashMap<(char, char), String>;
type Counts = HashMap<String, usize>;

#[allow(dead_code)]
fn do_it(part: Part, content: &str) -> usize {
    let (template_str, pair_str) = content.split_once("\n\n").unwrap();

    let template: Polymer = template_str.trim().chars().map(|c| c.to_string()).collect();

    let pairs: Lookup = pair_str
        .lines()
        .map(|p| {
            let x = p.split_once(" -> ").unwrap();
            let y: Vec<char> = x.0.to_string().chars().collect();
            ((y[0], y[1]), x.1.to_string())
        })
        .collect();

    let steps = match part {
        Part::One => 10,
        Part::Two => 40,
    };

    let mut counter: HashMap<(String, String), usize> = HashMap::new();
    for pair in template.windows(2) {
        *counter
            .entry((pair[0].to_string(), pair[1].to_string()))
            .or_insert(0) += 1;
    }

    for _step in 0..steps {
        let mut new_counter: HashMap<(String, String), usize> = HashMap::new();
        for pair in counter.keys() {
            let a = &pair.0;
            let c = &pair.1;
            let b = pairs
                .get(&(
                    a.chars().collect::<Vec<char>>()[0],
                    c.chars().collect::<Vec<char>>()[0],
                ))
                .unwrap();

            let ctr = counter.get(&(a.to_string(), c.to_string())).unwrap();
            *new_counter
                .entry((a.to_string(), b.to_string()))
                .or_insert(0) += ctr;
            *new_counter
                .entry((b.to_string(), c.to_string()))
                .or_insert(0) += ctr;
        }
        counter = new_counter;
    }

    let mut l_counts: Counts = HashMap::new();
    let mut r_counts: Counts = HashMap::new();

    for (pair, count) in &counter {
        *l_counts.entry(pair.0.to_string()).or_insert(0) += count;
        *r_counts.entry(pair.1.to_string()).or_insert(0) += count;
    }

    let mut counts: Counts = HashMap::new();

    for key in l_counts.keys().chain(r_counts.keys()) {
        let max_val = max(
            l_counts.get(key).unwrap_or(&0),
            r_counts.get(key).unwrap_or(&0),
        );
        *counts.entry(key.to_string()).or_insert(0) = *max_val;
    }

    counts.values().max().unwrap() - counts.values().min().unwrap()
}

#[cfg(test)] // cargo test -- --show-output [TEST_NAME]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("data/sample.txt");
    const INPUT: &str = include_str!("data/input.txt");

    #[test]
    fn part_1() {
        assert_eq!(do_it(Part::One, SAMPLE), 1588);
        assert_eq!(do_it(Part::One, INPUT), 3259);
    }

    #[test]
    fn part_2() {
        assert_eq!(do_it(Part::Two, SAMPLE), 2188189693529);
        assert_eq!(do_it(Part::Two, INPUT), 3459174981021);
    }
}
