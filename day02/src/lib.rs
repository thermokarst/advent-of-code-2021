#[allow(dead_code)]
enum Part {
    One,
    Two,
}

#[allow(dead_code)]
fn do_it(part: Part, content: &str) -> usize {
    let values: Vec<(_, usize)> = content
        .lines()
        .map(|val| {
            let parsed: Vec<_> = val.trim().split(' ').collect();
            (parsed[0], parsed[1].parse().unwrap())
        })
        .collect();

    let mut horiz: usize = 0;
    let mut depth: usize = 0;

    match part {
        Part::One => {
            for (inst, val) in values {
                match inst {
                    "forward" => horiz += val,
                    "down" => depth += val,
                    "up" => depth -= val,
                    _ => panic!("oops"),
                }
            }
        }
        Part::Two => {
            let mut aim: usize = 0;
            for (inst, val) in values {
                match inst {
                    "forward" => {
                        horiz += val;
                        depth += aim * val;
                    }
                    "down" => aim += val,
                    "up" => aim -= val,
                    _ => panic!("oops"),
                }
            }
        }
    }

    horiz * depth
}

#[cfg(test)] // cargo test -- --show-output [TEST_NAME]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("data/sample.txt");
    const INPUT: &str = include_str!("data/input.txt");

    #[test]
    fn part_1() {
        assert_eq!(do_it(Part::One, SAMPLE), 150);
        assert_eq!(do_it(Part::One, INPUT), 1693300);
    }

    #[test]
    fn part_2() {
        assert_eq!(do_it(Part::Two, SAMPLE), 900);
        assert_eq!(do_it(Part::Two, INPUT), 1857958050);
    }
}
