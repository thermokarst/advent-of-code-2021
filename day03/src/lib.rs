#[allow(dead_code)]
enum Part {
    One,
    Two,
}

fn bin_vec_to_dec(vec: &Vec<u32>) -> u32 {
    u32::from_str_radix(&vec.iter().map(|v| v.to_string()).collect::<String>(), 2).unwrap()
}

fn sum_vertically(values: &Vec<Vec<u32>>) -> Vec<u32> {
    values.iter().fold(vec![0; values[0].len()], |acc, line| {
        acc.iter().zip(line.iter()).map(|(&a, &b)| a + b).collect()
    })
}

fn part_one(values: &Vec<Vec<u32>>) -> u32 {
    let mut gamma = Vec::new();
    let mut epsilon = Vec::new();

    for value in sum_vertically(&values) {
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

    let gamma = bin_vec_to_dec(&gamma);
    let epsilon = bin_vec_to_dec(&epsilon);

    gamma * epsilon
}

fn flail_around(values: &Vec<Vec<u32>>, high: u32, low: u32) -> u32 {
    let mut candidates = values.clone();

    for pos in 0..candidates[0].len() {
        if candidates.len() == 1 {
            break;
        }

        let summed = sum_vertically(&candidates);
        let mid = (candidates.len() as f32 / 2.0).try_into().unwrap();
        let target = summed[pos] as f32;

        let matcher: u32;

        if target >= mid {
            matcher = high;
        } else {
            matcher = low;
        }

        candidates = candidates
            .into_iter()
            .filter(|v| v[pos] == matcher)
            .collect();
    }

    bin_vec_to_dec(&candidates[0])
}

fn part_two(values: &Vec<Vec<u32>>) -> u32 {
    let oxy = flail_around(&values, 1, 0);
    let co2 = flail_around(&values, 0, 1);

    oxy * co2
}

#[allow(dead_code)]
fn do_it(part: Part, content: &str) -> u32 {
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

    match part {
        Part::One => part_one(&values),
        Part::Two => part_two(&values),
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
        assert_eq!(do_it(Part::Two, SAMPLE), 230);
        assert_eq!(do_it(Part::Two, INPUT), 4481199);
    }
}
