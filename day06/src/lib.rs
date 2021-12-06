#[allow(dead_code)]
enum Part {
    One,
    Two,
}

type School = Vec<usize>;
type Counter = [usize; 9]; // fish ages are 0 - 8 days old, one el for each age group

#[allow(dead_code)]
fn do_it(part: Part, content: &str) -> usize {
    let school: School = content
        .lines()
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();

    let mut counter: Counter = school_to_counter(&school);

    match part {
        Part::One => {
            counter = exist(80, counter);
            count(counter)
        }
        Part::Two => {
            counter = exist(256, counter);
            count(counter)
        }
    }
}

fn school_to_counter(school: &School) -> Counter {
    let mut counter = [0, 0, 0, 0, 0, 0, 0, 0, 0];

    for fish in school {
        counter[*fish] = counter[*fish] + 1;
    }

    counter
}

fn exist(days: usize, mut counter: Counter) -> Counter {
    for _day in 0..days {
        let born = counter[0];
        counter.rotate_left(1); // age every group by one day
        counter[6] = counter[6] + born; // reset age of the new parent group
    }

    counter
}

fn count(counter: Counter) -> usize {
    counter.iter().fold(0, |mut acc, v| {
        acc += v;
        acc
    })
}

#[cfg(test)] // cargo test -- --show-output [TEST_NAME]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("data/sample.txt");
    const INPUT: &str = include_str!("data/input.txt");

    #[test]
    fn part_1() {
        assert_eq!(do_it(Part::One, SAMPLE), 5934);
        assert_eq!(do_it(Part::One, INPUT), 375482);
    }

    #[test]
    fn part_2() {
        assert_eq!(do_it(Part::Two, SAMPLE), 26984457539);
        assert_eq!(do_it(Part::Two, INPUT), 1689540415957);
    }
}
