#[allow(dead_code)]
enum Part {
    One,
    Two,
}

#[derive(Debug)]
struct Board {
    values: Vec<Vec<usize>>, // should probably use arrays, but whatever
    done: bool,
}

impl Board {
    fn from_lines(lines: [&str; 5]) -> Self {
        let mut values: Vec<Vec<usize>> = Vec::new();

        for line in lines.iter() {
            let row: Vec<usize> = line
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();
            values.push(row);
        }

        Self {
            values,
            done: false,
        }
    }

    fn check(&mut self, val: usize, marker: usize) -> bool {
        if self.done {
            return false;
        }

        for i in 0..self.values.len() {
            for j in 0..self.values[i].len() {
                if self.values[i][j] == val {
                    self.values[i][j] = marker;

                    if self.values[i].iter().all(|v| v == &marker) {
                        self.done = true;
                        return true;
                    }

                    if self.values.iter().map(|r| r[j]).all(|v| v == marker) {
                        self.done = true;
                        return true;
                    }
                }
            }
        }

        false
    }

    fn score(&self, call: usize, marker: usize) -> usize {
        let mut sum = 0;

        for i in 0..self.values.len() {
            for j in 0..self.values[i].len() {
                if self.values[i][j] != marker {
                    sum += self.values[i][j];
                }
            }
        }

        sum * call
    }
}

#[allow(dead_code)]
fn do_it(part: Part, content: &str) -> usize {
    let mut ls = content.lines();
    let queue: Vec<usize> = ls
        .next()
        .unwrap()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();

    ls.next(); // blank line between boards

    let mut boards: Vec<Board> = Vec::new();

    // this seems a bit ridiculous, but such is life
    while let (Some(l1), Some(l2), Some(l3), Some(l4), Some(l5)) =
        (ls.next(), ls.next(), ls.next(), ls.next(), ls.next())
    {
        let board = Board::from_lines([l1, l2, l3, l4, l5]);
        boards.push(board);
        ls.next(); // blank line between boards
    }

    // use an int not found in the queue as the marker
    let marker = queue.iter().max().unwrap() + 1;
    let mut finished_board_scores: Vec<usize> = Vec::new();

    for call in queue {
        for board in &mut boards {
            let bingo = board.check(call, marker);
            if bingo {
                finished_board_scores.push(board.score(call, marker));
            }
        }
    }

    match part {
        Part::One => *finished_board_scores.first().unwrap(),
        Part::Two => *finished_board_scores.last().unwrap(),
    }
}

#[cfg(test)] // cargo test -- --show-output [TEST_NAME]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("data/sample.txt");
    const INPUT: &str = include_str!("data/input.txt");

    #[test]
    fn part_1() {
        assert_eq!(do_it(Part::One, SAMPLE), 4512);
        assert_eq!(do_it(Part::One, INPUT), 71708);
    }

    #[test]
    fn part_2() {
        assert_eq!(do_it(Part::Two, SAMPLE), 1924);
        assert_eq!(do_it(Part::Two, INPUT), 34726);
    }
}
