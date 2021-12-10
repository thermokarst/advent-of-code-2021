#[allow(dead_code)]
enum Part {
    One,
    Two,
}

type Heightmap = Vec<Vec<u32>>;

#[allow(dead_code)]
fn do_it(part: Part, content: &str) -> u32 {
    let height_map: Heightmap = content
        .lines()
        .map(|l| l.chars().map(|v| v.to_digit(10).unwrap()).collect())
        .collect();

    let n_rows = height_map.len();
    let n_cols = height_map[0].len();

    let mut padded: Heightmap = vec![vec![9; n_cols + 2]];
    for mut row in height_map {
        row.push(9);
        let mut new_row = vec![9];
        new_row.extend_from_slice(&row);
        padded.push(new_row);
    }
    padded.push(vec![9; n_cols + 2]);

    let mut risk = 0;
    let mut basins: Vec<u32> = Vec::new();

    for row in 1..n_rows + 1 {
        for col in 1..n_cols + 1 {
            let up = padded[row - 1][col];
            let left = padded[row][col - 1];
            let val = padded[row][col];
            let right = padded[row][col + 1];
            let down = padded[row + 1][col];

            if val < up && val < left && val < right && val < down {
                risk += 1 + val;
                let area = basin_area(&mut padded, row, col);
                basins.push(area)
            }
        }
    }

    match part {
        Part::One => risk,
        Part::Two => {
            basins.sort_by(|a, b| b.cmp(a));
            basins[0] * basins[1] * basins[2]
        }
    }
}

fn basin_area(map: &mut Heightmap, row: usize, col: usize) -> u32 {
    let val = map[row][col];
    map[row][col] = 9;
    let mut size = 1;

    // left
    let left = col - 1;
    if map[row][left] >= val && map[row][left] < 9 {
        size += basin_area(map, row, left);
    }

    // right
    let right = col + 1;
    if map[row][right] >= val && map[row][right] < 9 {
        size += basin_area(map, row, right);
    }

    // up
    let up = row - 1;
    if map[up][col] >= val && map[up][col] < 9 {
        size += basin_area(map, up, col);
    }

    // down
    let down = row + 1;
    if map[down][col] >= val && map[down][col] < 9 {
        size += basin_area(map, down, col);
    }

    size
}

#[cfg(test)] // cargo test -- --show-output [TEST_NAME]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("data/sample.txt");
    const INPUT: &str = include_str!("data/input.txt");

    #[test]
    fn part_1() {
        assert_eq!(do_it(Part::One, SAMPLE), 15);
        assert_eq!(do_it(Part::One, INPUT), 458);
    }

    #[test]
    fn part_2() {
        assert_eq!(do_it(Part::Two, SAMPLE), 1134);
        assert_eq!(do_it(Part::Two, INPUT), 1391940);
    }
}
