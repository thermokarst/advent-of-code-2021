use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
enum Part {
    One,
    Two,
}

#[derive(Debug)]
enum State {
    On,
    Off,
}

type Range = (isize, isize);

#[derive(Debug)]
struct RebootStep {
    state: State,
    x: Range,
    y: Range,
    z: Range,
}

type Coord = (isize, isize, isize);

#[allow(dead_code)]
fn do_it(part: Part, content: &str) -> usize {
    let mut x_ranges: HashSet<isize> = HashSet::new();
    let mut y_ranges: HashSet<isize> = HashSet::new();
    let mut z_ranges: HashSet<isize> = HashSet::new();

    let mut instructions: Vec<RebootStep> = content
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(' ').collect();
            let state = match parts[0] {
                "on" => State::On,
                "off" => State::Off,
                _ => unreachable!(),
            };
            let ranges: Vec<&str> = parts[1].split(',').collect();
            let x: Vec<isize> = ranges[0]
                .trim_start_matches("x=")
                .split("..")
                .map(|v| v.parse().unwrap())
                .collect();
            let y: Vec<isize> = ranges[1]
                .trim_start_matches("y=")
                .split("..")
                .map(|v| v.parse().unwrap())
                .collect();
            let z: Vec<isize> = ranges[2]
                .trim_start_matches("z=")
                .split("..")
                .map(|v| v.parse().unwrap())
                .collect();

            x_ranges.insert(x[0]);
            x_ranges.insert(x[1] + 1);
            y_ranges.insert(y[0]);
            y_ranges.insert(y[1] + 1);
            z_ranges.insert(z[0]);
            z_ranges.insert(z[1] + 1);

            RebootStep {
                state,
                x: (x[0], x[1] + 1),
                y: (y[0], y[1] + 1),
                z: (z[0], z[1] + 1),
            }
        })
        .collect();

    match part {
        Part::One => {
            let mut coords: HashSet<Coord> = HashSet::new();

            for instruction in instructions.iter_mut() {
                if instruction.x.0 > 50
                    || instruction.x.1 < -50 && instruction.y.0 > 50
                    || instruction.y.1 < -50
                    || instruction.z.0 > 50
                    || instruction.z.1 < -50
                {
                    continue;
                }

                if instruction.x.0 < -50 {
                    instruction.x.0 = -50;
                }

                if instruction.x.1 > 51 {
                    instruction.x.1 = 51;
                }

                if instruction.y.0 < -50 {
                    instruction.y.0 = -50;
                }

                if instruction.y.1 > 51 {
                    instruction.y.1 = 51;
                }

                if instruction.z.0 < -50 {
                    instruction.z.0 = -50;
                }

                if instruction.z.1 > 51 {
                    instruction.z.1 = 51;
                }

                for x in instruction.x.0..instruction.x.1 {
                    for y in instruction.y.0..instruction.y.1 {
                        for z in instruction.z.0..instruction.z.1 {
                            match instruction.state {
                                State::On => coords.insert((x, y, z)),
                                State::Off => coords.remove(&(x, y, z)),
                            };
                        }
                    }
                }
            }

            coords.len()
        }

        Part::Two => {
            let mut x_ranges = x_ranges.into_iter().collect::<Vec<_>>();
            let mut y_ranges = y_ranges.into_iter().collect::<Vec<_>>();
            let mut z_ranges = z_ranges.into_iter().collect::<Vec<_>>();

            x_ranges.sort_unstable();
            y_ranges.sort_unstable();
            z_ranges.sort_unstable();

            let mapit = |mut acc: HashMap<isize, usize>, (i, v)| {
                acc.insert(v, i);
                acc
            };
            let xr = x_ranges
                .clone()
                .into_iter()
                .enumerate()
                .fold(HashMap::new(), mapit);
            let yr = y_ranges
                .clone()
                .into_iter()
                .enumerate()
                .fold(HashMap::new(), mapit);
            let zr = z_ranges
                .clone()
                .into_iter()
                .enumerate()
                .fold(HashMap::new(), mapit);

            let mut reactor: Vec<Vec<Vec<bool>>> =
                vec![vec![vec![false; zr.len()]; yr.len()]; xr.len()];

            for instruction in instructions.iter_mut() {
                for x in (xr[&instruction.x.0] as usize)..(xr[&instruction.x.1] as usize) {
                    for y in (yr[&instruction.y.0] as usize)..(yr[&instruction.y.1] as usize) {
                        for z in (zr[&instruction.z.0] as usize)..(zr[&instruction.z.1] as usize) {
                            reactor[x][y][z] = match instruction.state {
                                State::On => true,
                                State::Off => false,
                            };
                        }
                    }
                }
            }

            let mut count = 0;
            for x in 0..reactor.len() {
                for y in 0..reactor[x].len() {
                    for z in 0..reactor[x][y].len() {
                        if reactor[x][y][z] {
                            count += (x_ranges[x + 1] - x_ranges[x])
                                * (y_ranges[y + 1] - y_ranges[y])
                                * (z_ranges[z + 1] - z_ranges[z]);
                        }
                    }
                }
            }

            count as usize
        }
    }
}

#[cfg(test)] // cargo test -- --show-output [TEST_NAME]
mod tests {
    use super::*;

    const SAMPLE1: &str = include_str!("data/sample1.txt");
    const SAMPLE2: &str = include_str!("data/sample2.txt");
    const INPUT: &str = include_str!("data/input.txt");

    #[test]
    fn part_1() {
        assert_eq!(do_it(Part::One, SAMPLE1), 590784);
        assert_eq!(do_it(Part::One, INPUT), 647076);
    }

    #[test]
    fn part_2() {
        assert_eq!(do_it(Part::Two, SAMPLE2), 2758514936282235);
        assert_eq!(do_it(Part::Two, INPUT), 1233304599156793);
    }
}
