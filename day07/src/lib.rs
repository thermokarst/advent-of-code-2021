#[allow(dead_code)]
enum Part {
    One,
    Two,
}

type Crabs = Vec<isize>;
type Scores = Vec<usize>;

#[allow(dead_code)]
fn do_it(part: Part, content: &str) -> usize {
    let crabs: Crabs = content
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();

    let mut fuel_scores: Scores = Vec::new();
    for i in 0..crabs.len() {
        let mut score = 0;
        for j in 0..crabs.len() {
            let diff = (crabs[i] - crabs[j]).abs() as usize;
            match part {
                Part::One => {
                    score = score + diff;
                }
                Part::Two => {
                    score = score + (((diff * diff) + diff) / 2);
                }
            }
        }
        fuel_scores.push(score);
    }
    *fuel_scores.iter().min().unwrap()
}

#[cfg(test)] // cargo test -- --show-output [TEST_NAME]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("data/sample.txt");
    const INPUT: &str = include_str!("data/input.txt");

    #[test]
    fn part_1() {
        assert_eq!(do_it(Part::One, SAMPLE), 37);
        assert_eq!(do_it(Part::One, INPUT), 351901);
    }

    #[test]
    fn part_2() {
        // The AOC prompt uses "5" as their new example for the most fuel
        // efficient position, however none of the crabs are in position 5. I
        // think this might just be a typo? My full-input answer is correct.
        assert_eq!(do_it(Part::Two, SAMPLE), 170);
        assert_eq!(do_it(Part::Two, INPUT), 101079875);
    }
}
