use std::cmp::max;
use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
enum Part {
    One,
    Two,
}

type Coord = (isize, isize, isize);
type ScannerSet = HashSet<Coord>;

fn find_rotations(mut scanner_set: ScannerSet) -> Vec<ScannerSet> {
    let mut rotations: Vec<ScannerSet> = Vec::new();

    for _i in 0..4 {
        for _j in 0..4 {
            rotations.push(scanner_set.clone());
            let mut tmp_scanner_set = ScannerSet::new();
            for (x, y, z) in scanner_set {
                tmp_scanner_set.insert((z, y, -x));
            }
            scanner_set = tmp_scanner_set;
        }

        let mut tmp_a = ScannerSet::new();
        let mut tmp_b = ScannerSet::new();

        for (x, y, z) in &scanner_set {
            tmp_a.insert((*y, -x, *z));
            tmp_b.insert((-y, *x, *z));
        }
        rotations.push(tmp_a);
        rotations.push(tmp_b);

        let mut tmp_scanner_set = ScannerSet::new();
        for (x, y, z) in scanner_set {
            tmp_scanner_set.insert((x, z, -y));
        }
        scanner_set = tmp_scanner_set;
    }

    rotations
}

fn intersect(set_a: ScannerSet, set_b: ScannerSet) -> Option<(ScannerSet, Coord)> {
    for rotation in find_rotations(set_b) {
        for a in &set_a {
            for b in &rotation {
                let offset = (b.0 - a.0, b.1 - a.1, b.2 - a.2);
                let mut tmp: ScannerSet = ScannerSet::new();
                for c in &rotation {
                    tmp.insert((c.0 - offset.0, c.1 - offset.1, c.2 - offset.2));
                }

                let intersection = tmp.intersection(&set_a);
                if intersection.count() > 11 {
                    return Some((tmp, offset));
                }
            }
        }
    }

    None
}

#[allow(dead_code)]
fn do_it(part: Part, content: &str) -> usize {
    let mut scanners: Vec<ScannerSet> = Vec::new();

    for scanner in content.split("\n\n") {
        let mut scanner_set = ScannerSet::new();
        for line in scanner.lines().skip(1) {
            let tmp: Vec<isize> = line.split(',').map(|v| v.parse().unwrap()).collect();
            scanner_set.insert((tmp[0], tmp[1], tmp[2]));
        }
        scanners.push(scanner_set);
    }

    let mut probes = scanners[0].clone();
    let mut offsets: Vec<Coord> = Vec::from([(0, 0, 0)]);
    let mut remaining = VecDeque::new();
    for scanner in scanners.iter().skip(1) {
        remaining.push_back(scanner.clone());
    }

    while !remaining.is_empty() {
        let check = remaining.pop_front().unwrap();
        if let Some((int, offset)) = intersect(probes.clone(), check.clone()) {
            for probe in int {
                probes.insert(probe);
            }
            offsets.push(offset);
        } else {
            remaining.push_back(check);
        }
    }

    let mut distance = 0;

    for from in &offsets {
        for to in &offsets {
            distance = max(
                distance,
                (from.0 - to.0).abs() + (from.1 - to.1).abs() + (from.2 - to.2).abs(),
            );
        }
    }

    match part {
        Part::One => probes.len(),
        Part::Two => distance.try_into().unwrap(),
    }
}

#[cfg(test)] // cargo test -- --show-output [TEST_NAME]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("data/sample.txt");
    const INPUT: &str = include_str!("data/input.txt");

    #[test]
    fn part_1() {
        assert_eq!(do_it(Part::One, SAMPLE), 79);
        assert_eq!(do_it(Part::One, INPUT), 472);
    }

    #[test]
    fn part_2() {
        assert_eq!(do_it(Part::Two, SAMPLE), 3621);
        assert_eq!(do_it(Part::Two, INPUT), 12092);
    }
}
