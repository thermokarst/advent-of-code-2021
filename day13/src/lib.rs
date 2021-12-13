#[allow(dead_code)]
enum Part {
    One,
    Two,
}

type Coord = (usize, usize);
type Grid = Vec<Vec<usize>>;

#[derive(Debug, PartialEq)]
enum Axis {
    X,
    Y,
}

#[derive(Debug)]
struct Fold {
    axis: Axis,
    pos: usize,
}

impl Fold {
    fn new(line: &str) -> Self {
        let (axis_str, pos_str) = line.split_once('=').unwrap();
        let axis = match axis_str {
            "fold along x" => Axis::X,
            "fold along y" => Axis::Y,
            _ => panic!("oops"),
        };
        let pos = pos_str.parse().unwrap();

        Self { axis, pos }
    }
}

fn print_grid(grid: &[Vec<usize>]) {
    for row in grid {
        for val in row {
            if val > &0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

#[allow(dead_code)]
fn do_it(part: Part, content: &str) -> usize {
    let (coords_str, folds_str) = content.split_once("\n\n").unwrap();

    let coords: Vec<Coord> = coords_str
        .lines()
        .map(|v| {
            let pair: Vec<_> = v.split(',').collect();
            (pair[0].parse().unwrap(), pair[1].parse().unwrap())
        })
        .collect();

    let folds: Vec<Fold> = folds_str.lines().map(|l| Fold::new(l)).collect();

    let cols = coords.iter().max_by_key(|(a, _b)| a).unwrap().0;
    let rows = coords.iter().max_by_key(|(_a, b)| b).unwrap().1;

    let mut grid: Grid = vec![vec![0; cols + 1]; rows + 1];

    for (x, y) in &coords {
        grid[*y][*x] = 1;
    }

    let mut dots: Vec<usize> = Vec::new();

    for fold in folds {
        let (width, height): (usize, usize) = match fold.axis {
            Axis::X => (fold.pos, grid.len()),
            Axis::Y => (grid[0].len(), fold.pos),
        };

        let mut folded_grid: Grid = vec![vec![0; width]; height];

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if i < height && j < width {
                    folded_grid[i][j] += grid[i][j];
                } else if i > height && fold.axis == Axis::Y {
                    let mirror_i = 2 * fold.pos - i;
                    folded_grid[mirror_i][j] += grid[i][j];
                } else if j > width && fold.axis == Axis::X {
                    let mirror_j = 2 * fold.pos - j;
                    folded_grid[i][mirror_j] += grid[i][j];
                }
            }
        }

        let dot_count: usize = folded_grid
            .iter()
            .map(|row| row.iter().filter(|&v| v > &0).count())
            .sum();

        dots.push(dot_count);

        grid = folded_grid;
    }

    match part {
        Part::One => dots[0],
        Part::Two => {
            print_grid(&grid);
            *dots.last().unwrap()
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
        assert_eq!(do_it(Part::One, SAMPLE), 17);
        assert_eq!(do_it(Part::One, INPUT), 747);
    }

    #[test]
    fn part_2() {
        assert_eq!(do_it(Part::Two, SAMPLE), 16); // O
        assert_eq!(do_it(Part::Two, INPUT), 102); // ARHZPCUH
    }
}
