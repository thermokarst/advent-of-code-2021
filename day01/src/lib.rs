use std::fs;

pub enum Part {
    One,
    Two,
}

pub fn do_it(part: Part, filename: String) -> usize {
    let contents = fs::read_to_string(filename).unwrap();
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

    #[test]
    fn sample_part_1() {
        assert_eq!(do_it(Part::One, "data/sample.txt".to_string()), 7);
    }

    #[test]
    fn sample_part_2() {
        assert_eq!(do_it(Part::Two, "data/sample.txt".to_string()), 5);
    }

    #[test]
    fn part_1() {
        assert_eq!(do_it(Part::One, "data/input.txt".to_string()), 1374);
    }

    #[test]
    fn part_2() {
        assert_eq!(do_it(Part::Two, "data/input.txt".to_string()), 1418);
    }
}
