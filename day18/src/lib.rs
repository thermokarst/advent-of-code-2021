use std::cmp::max;
use std::fmt;

#[allow(dead_code)]
enum Part {
    One,
    Two,
}

enum Side {
    Left,
    Right,
}

#[derive(Clone)]
enum Cell {
    Value(usize),
    Pair(Box<Cell>, Box<Cell>),
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Cell::Value(x) => write!(f, "{}", x),
            Cell::Pair(y, z) => write!(f, "[{},{}]", y, z),
        }
    }
}

impl Cell {
    fn parse(val: &[char], pos: usize) -> (usize, Self) {
        match val[pos] {
            '[' => {
                let (pos, cell1) = Self::parse(val, pos + 1);
                let (pos, cell2) = Self::parse(val, pos + 1);
                (pos + 1, Self::Pair(Box::new(cell1), Box::new(cell2)))
            }
            v => (
                pos + 1,
                Cell::Value(v.to_digit(10).unwrap().try_into().unwrap()),
            ),
        }
    }

    fn reduce(mut self) -> Self {
        loop {
            let depth = self.depth();
            if depth > 4 {
                if let Some((_, cell, _)) = self.explode_pair(depth) {
                    self = cell;
                    continue;
                }
            }

            match self.split_value() {
                Some(cell) => {
                    self = cell;
                }
                None => {
                    return self;
                }
            }
        }
    }

    fn depth(&self) -> usize {
        match self {
            Self::Value(_) => 0,
            // go with which ever side is deeper
            Self::Pair(left, right) => 1 + max(left.depth(), right.depth()),
        }
    }

    fn add_parent(&self, cell: Option<Self>, side: Side) -> Self {
        match (self, cell) {
            (Self::Value(left), Some(Self::Value(right))) => Self::Value(*left + right),
            (Self::Pair(left, right), cell) => match side {
                Side::Left => {
                    Self::Pair(left.clone(), Box::new(right.add_parent(cell, Side::Left)))
                }
                Side::Right => {
                    Self::Pair(Box::new(left.add_parent(cell, Side::Right)), right.clone())
                }
            },
            (_, None) => self.clone(),
            _ => unreachable!(),
        }
    }

    fn explode_pair(&self, depth: usize) -> Option<(Option<Self>, Self, Option<Self>)> {
        if let Self::Pair(left, right) = self {
            // if depth is 1, then we can set this cell to 0
            if depth == 1 {
                return Some((Some(*left.clone()), Self::Value(0), Some(*right.clone())));
            }

            // add left to parent, if necessary
            if let Some((left_parent, cell, right_parent)) = left.explode_pair(depth - 1) {
                let summed = right.add_parent(right_parent, Side::Right);
                let new_cell = Self::Pair(Box::new(cell), Box::new(summed));
                return Some((left_parent, new_cell, None));
            }

            // add right to parent, if necessary
            if let Some((left_parent, cell, right_parent)) = right.explode_pair(depth - 1) {
                let summed = left.add_parent(left_parent, Side::Left);
                let new_cell = Self::Pair(Box::new(summed), Box::new(cell));
                return Some((None, new_cell, right_parent));
            }
        }

        None
    }

    fn split_value(&self) -> Option<Self> {
        match self {
            Self::Value(value) => {
                if *value >= 10 {
                    let split_value = *value as f32 / 2.0;
                    let left = split_value.floor() as usize;
                    let right = split_value.ceil() as usize;

                    return Some(Self::Pair(
                        Box::new(Self::Value(left)),
                        Box::new(Self::Value(right)),
                    ));
                }
            }

            Self::Pair(left, right) => {
                if let Some(cell) = left.split_value() {
                    return Some(Self::Pair(Box::new(cell), right.clone()));
                }

                if let Some(cell) = right.split_value() {
                    return Some(Self::Pair(left.clone(), Box::new(cell)));
                }
            }
        }

        None
    }

    fn sum_numbers(self, other: Self) -> Self {
        let reduced_self = self.reduce();
        let reduced_other = other.reduce();
        let summed = Self::Pair(Box::new(reduced_self), Box::new(reduced_other));

        summed.reduce()
    }

    fn magnitude(&self) -> usize {
        match self {
            Self::Value(value) => *value,
            Self::Pair(left, right) => (3 * left.magnitude()) + (2 * right.magnitude()),
        }
    }
}

#[allow(dead_code)]
fn do_it(part: Part, content: &str) -> usize {
    let numbers: Vec<Cell> = content
        .lines()
        .map(|l| {
            let c: Vec<char> = l.chars().collect();
            let (_, cell) = Cell::parse(&c, 0);
            cell
        })
        .collect();

    match part {
        Part::One => {
            let mut a = numbers[0].clone();
            for b in numbers.iter().skip(1) {
                a = a.sum_numbers(b.clone());
            }

            a.magnitude()
        }
        Part::Two => {
            let mut max_mag = 0;
            for i in 0..numbers.len() {
                for j in 0..numbers.len() {
                    let a = numbers[i].clone();
                    let b = numbers[j].clone();

                    let summed_fwd = a.sum_numbers(b);
                    let fwd_mag = summed_fwd.magnitude();
                    if fwd_mag > max_mag {
                        max_mag = fwd_mag;
                    }

                    let a = numbers[i].clone();
                    let b = numbers[j].clone();

                    let summed_rev = b.sum_numbers(a);
                    let rev_mag = summed_rev.magnitude();
                    if rev_mag > max_mag {
                        max_mag = rev_mag;
                    }
                }
            }
            max_mag
        }
    }
}

#[cfg(test)] // cargo test -- --show-output [TEST_NAME]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("data/sample.txt");
    const INPUT: &str = include_str!("data/input.txt");

    #[test]
    fn part_1() {
        assert_eq!(do_it(Part::One, SAMPLE), 4140);
        assert_eq!(do_it(Part::One, INPUT), 4033);
    }

    #[test]
    fn part_2() {
        assert_eq!(do_it(Part::Two, SAMPLE), 3993);
        assert_eq!(do_it(Part::Two, INPUT), 4864);
    }
}
