#[allow(dead_code)]
enum Part {
    One,
    Two,
}

#[allow(dead_code)]
fn do_it(part: Part, contents: &str) -> usize {
    let mut values: Vec<usize> = contents
        .lines()
        .map(|val| val.trim().parse().unwrap())
        .collect();

    if let Part::Two = part {
        values = values.windows(3).map(|w| w.iter().sum()).collect();
    }

    values.windows(2).map(|w| (w[1] > w[0]) as usize).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("data/sample.txt");
    const INPUT: &str = include_str!("data/input.txt");

    #[test]
    fn part_1() {
        assert_eq!(do_it(Part::One, SAMPLE), 7);
        assert_eq!(do_it(Part::One, INPUT), 1374);
    }

    #[test]
    fn part_2() {
        assert_eq!(do_it(Part::Two, SAMPLE), 5);
        assert_eq!(do_it(Part::Two, INPUT), 1418);
    }
}
