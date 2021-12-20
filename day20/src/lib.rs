const PAD: usize = 4;

#[allow(dead_code)]
enum Part {
    One,
    Two,
}

fn parse(c: char) -> usize {
    match c {
        '.' => 0,
        '#' => 1,
        _ => unreachable!(),
    }
}

fn pad(input: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut padded: Vec<Vec<usize>> =
        vec![vec![0; input.len() + (2 * PAD)]; input[0].len() + (2 * PAD)];

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            padded[i + PAD][j + PAD] = input[i][j];
        }
    }
    padded
}

fn depad(input: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut depadded: Vec<Vec<usize>> =
        vec![vec![0; input.len() - (2 * PAD)]; input[0].len() - (2 * PAD)];

    for i in PAD..input.len() - PAD {
        for j in PAD..input[i].len() - PAD {
            depadded[i - PAD][j - PAD] = input[i][j];
        }
    }
    depadded
}

fn print(input: &[Vec<usize>]) {
    for row in input {
        for c in row {
            match c {
                0 => print!("."),
                1 => print!("#"),
                _ => unreachable!(),
            }
        }
        println!();
    }
}

#[allow(dead_code)]
fn do_it(part: Part, content: &str) -> usize {
    let (algo_str, input_str) = content.split_once("\n\n").unwrap();
    let algo: Vec<usize> = algo_str.chars().map(parse).collect();
    let mut padded: Vec<Vec<usize>> = input_str
        .lines()
        .map(|l| l.chars().map(parse).collect())
        .collect();

    let rounds = match part {
        Part::One => 1,
        Part::Two => 25,
    };

    for _base in 0..rounds {
        for _round in 0..2 {
            let tmp = pad(padded.clone());
            padded = vec![vec![0; tmp.len() - 2]; tmp[0].len() - 2];
            for x in 1..tmp.len() - 1 {
                for y in 1..tmp[x].len() - 1 {
                    let a = tmp[x - 1][y - 1];
                    let b = tmp[x - 1][y];
                    let c = tmp[x - 1][y + 1];
                    let d = tmp[x][y - 1];
                    let e = tmp[x][y];
                    let f = tmp[x][y + 1];
                    let g = tmp[x + 1][y - 1];
                    let h = tmp[x + 1][y];
                    let i = tmp[x + 1][y + 1];

                    let binary_str = format!("{}{}{}{}{}{}{}{}{}", a, b, c, d, e, f, g, h, i);
                    let summed = usize::from_str_radix(&binary_str, 2).unwrap();

                    padded[x - 1][y - 1] = algo[summed];
                }
            }
        }
        padded = depad(padded);
    }

    print(&padded);

    padded.iter().map(|row| row.iter().sum::<usize>()).sum()
}

#[cfg(test)] // cargo test -- --show-output [TEST_NAME]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("data/sample.txt");
    const INPUT: &str = include_str!("data/input.txt");

    #[test]
    fn part_1() {
        assert_eq!(do_it(Part::One, SAMPLE), 35);
        assert_eq!(do_it(Part::One, INPUT), 5846);
    }

    #[test]
    fn part_2() {
        assert_eq!(do_it(Part::Two, SAMPLE), 3351);
        assert_eq!(do_it(Part::Two, INPUT), 21149);
    }
}
