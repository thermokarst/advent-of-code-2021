#[allow(dead_code)]
enum Part {
    One,
    Two,
}

fn bin_vec_to_dec(vec: Vec<usize>) -> usize {
    usize::from_str_radix(&vec.iter().map(|v| v.to_string()).collect::<String>(), 2).unwrap()
}

#[allow(dead_code)]
fn do_it(part: Part, content: &str) -> usize {
    let values: Vec<Vec<_>> = content
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .collect::<Vec<_>>()
                .into_iter()
                .map(|v| v.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let summed = values.iter().fold(vec![0; values[0].len()], |acc, line| {
        acc.iter().zip(line.iter()).map(|(&a, &b)| a + b).collect()
    });

    let mut gamma = Vec::new();
    let mut epsilon = Vec::new();

    for value in summed {
        match value > (values.len() / 2).try_into().unwrap() {
            true => {
                gamma.push(1);
                epsilon.push(0);
            }
            false => {
                gamma.push(0);
                epsilon.push(1);
            }
        }
    }

    let gamma = bin_vec_to_dec(gamma);
    let epsilon = bin_vec_to_dec(epsilon);

    match part {
        Part::One => gamma * epsilon,
        Part::Two => 2,
    }
}

#[cfg(test)] // cargo test -- --show-output [TEST_NAME]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("data/sample.txt");
    const INPUT: &str = include_str!("data/input.txt");

    #[test]
    fn part_1() {
        assert_eq!(do_it(Part::One, SAMPLE), 198);
        assert_eq!(do_it(Part::One, INPUT), 3320834);
    }

    #[test]
    fn part_2() {
        assert_eq!(do_it(Part::Two, SAMPLE), 2);
        assert_eq!(do_it(Part::Two, INPUT), 2);
    }
}
