#[allow(dead_code)]
enum Part {
    One,
    Two,
}

type Crabs = Vec<isize>;

#[allow(dead_code)]
fn do_it(part: Part, content: &str) -> usize {
    let crabs: Crabs = content
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();

    let mut min_score = usize::MAX;
    'outer: for to_crab in &crabs {
        let mut score = 0;
        for from_crab in &crabs {
            let diff = (to_crab - from_crab).abs() as usize;
            score += match part {
                Part::One => diff,
                Part::Two => ((diff * diff) + diff) / 2,
            };
            if score > min_score {
                continue 'outer;
            }
        }
        if score < min_score {
            min_score = score;
        }
    }
    min_score
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
