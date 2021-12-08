#[allow(dead_code)]
enum Part {
    One,
    Two,
}

type Instructions = Vec<Line>;
type Line = ((usize, usize), (usize, usize));
type Board = Vec<Vec<usize>>;

#[allow(dead_code)]
fn do_it(part: Part, content: &str) -> usize {
    let all_lines: Instructions = content
        .lines()
        .map(|line| {
            let mut parts = line.split(" -> ");
            let (mut start_str, mut end_str) = (
                parts.next().unwrap().split(','),
                parts.next().unwrap().split(','),
            );
            let start = (
                start_str.next().unwrap().parse().unwrap(),
                start_str.next().unwrap().parse().unwrap(),
            );
            let end = (
                end_str.next().unwrap().parse().unwrap(),
                end_str.next().unwrap().parse().unwrap(),
            );

            (start, end)
        })
        .collect();

    let ortho_lines: Vec<_> = all_lines
        .clone()
        .into_iter()
        .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
        .collect();

    match part {
        Part::One => {
            let (width, height) = board_size(&ortho_lines);
            let ortho_board: Board = vec![vec![0; width + 1]; height + 1];
            let ortho_board = tally(ortho_board, ortho_lines, true);
            count(ortho_board)
        }
        Part::Two => {
            let (width, height) = board_size(&all_lines);
            let full_board: Board = vec![vec![0; width + 1]; height + 1];
            let full_board = tally(full_board, all_lines, false);
            count(full_board)
        }
    }
}

fn board_size(lines: &[Line]) -> (usize, usize) {
    let mut width = 0;
    let mut height = 0;

    for ((x1, y1), (x2, y2)) in lines.iter() {
        if x1 > &width {
            width = *x1;
        }
        if x2 > &width {
            width = *x2;
        }
        if y1 > &height {
            height = *y1;
        }
        if y2 > &height {
            height = *y2;
        }
    }

    (width, height)
}

fn tally(mut board: Board, lines: Instructions, only_ortho: bool) -> Board {
    for ((x1, y1), (x2, y2)) in lines.iter() {
        let min_x = std::cmp::min(x1, x2);
        let max_x = std::cmp::max(x1, x2) + 1;
        let min_y = std::cmp::min(y1, y2);
        let max_y = std::cmp::max(y1, y2) + 1;

        if x1 == x2 {
            // vertical line
            for i in board.iter_mut().take(max_y).skip(*min_y) {
                i[*x1] += 1;
            }
        } else if y1 == y2 {
            // horizontal line
            for i in *min_x..max_x {
                board[*y1][i] += 1;
            }
        } else if !only_ortho {
            // diagonal line
            let mut downward_slope = true;
            if (x1 < x2) && (y1 > y2) || (x2 < x1) && (y2 > y1) {
                downward_slope = false;
            }

            if downward_slope {
                for (ii, i) in (*min_x..max_x).enumerate() {
                    for (jj, j) in (*min_y..max_y).enumerate() {
                        if ii == jj {
                            board[j][i] += 1;
                        }
                    }
                }
            } else {
                for (ii, i) in (*min_x..max_x).enumerate() {
                    for (jj, j) in (*min_y..max_y).rev().enumerate() {
                        if ii == jj {
                            board[j][i] += 1;
                        }
                    }
                }
            }
        }
    }

    board
}

fn count(board: Board) -> usize {
    let mut count = 0;
    for i in &board {
        for j in i {
            if j > &1 {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)] // cargo test -- --show-output [TEST_NAME]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("data/sample.txt");
    const INPUT: &str = include_str!("data/input.txt");

    #[test]
    fn part_1() {
        assert_eq!(do_it(Part::One, SAMPLE), 5);
        assert_eq!(do_it(Part::One, INPUT), 7297);
    }

    #[test]
    fn part_2() {
        assert_eq!(do_it(Part::Two, SAMPLE), 12);
        assert_eq!(do_it(Part::Two, INPUT), 21038);
    }
}
