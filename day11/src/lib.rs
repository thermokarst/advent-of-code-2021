#[allow(dead_code)]
enum Part {
    One,
    Two,
}

type Octopi = Vec<Vec<u32>>;

#[allow(dead_code)]
fn do_it(part: Part, content: &str) -> usize {
    let mut octopi: Octopi = content
        .lines()
        .map(|l| l.chars().map(|v| v.to_digit(10).unwrap()).collect())
        .collect();

    match part {
        Part::One => {
            let mut flashes = 0;
            for _ in 0..100 {
                increase_all(&mut octopi);
                let round_flash = flash(&mut octopi);
                flashes += round_flash;
            }

            flashes
        }
        Part::Two => {
            let mut round = 0;
            let mut round_flash = 0;
            while round_flash != 100 {
                increase_all(&mut octopi);
                round_flash = flash(&mut octopi);
                round += 1;
            }

            round
        }
    }
}

fn flash(octopi: &mut Octopi) -> usize {
    let mut flashes = 0;

    for i in 0..octopi.len() {
        for j in 0..octopi[i].len() {
            if octopi[i][j] > 9 {
                octopi[i][j] = 0;

                // upper left
                if i > 0 && j > 0 && octopi[i - 1][j - 1] > 0 {
                    octopi[i - 1][j - 1] += 1;
                }

                // upper middle
                if i > 0 && octopi[i - 1][j] > 0 {
                    octopi[i - 1][j] += 1;
                }

                // upper right
                if i > 0 && j < octopi[i].len() - 1 && octopi[i - 1][j + 1] > 0 {
                    octopi[i - 1][j + 1] += 1;
                }

                // left
                if j > 0 && octopi[i][j - 1] > 0 {
                    octopi[i][j - 1] += 1;
                }

                // right
                if j < octopi[i].len() - 1 && octopi[i][j + 1] > 0 {
                    octopi[i][j + 1] += 1;
                }

                // lower left
                if i < octopi.len() - 1 && j > 0 && octopi[i + 1][j - 1] > 0 {
                    octopi[i + 1][j - 1] += 1;
                }

                // lower center
                if i < octopi.len() - 1 && octopi[i + 1][j] > 0 {
                    octopi[i + 1][j] += 1;
                }

                // lower right
                if i < octopi.len() - 1 && j < octopi[i].len() - 1 && octopi[i + 1][j + 1] > 0 {
                    octopi[i + 1][j + 1] += 1;
                }

                let round_flashes = flash(octopi);
                flashes += round_flashes + 1;
            }
        }
    }

    flashes
}

fn increase_all(octopi: &mut Octopi) {
    for row in octopi {
        for octopus in row {
            *octopus += 1;
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
        assert_eq!(do_it(Part::One, SAMPLE), 1656);
        assert_eq!(do_it(Part::One, INPUT), 1691);
    }

    #[test]
    fn part_2() {
        assert_eq!(do_it(Part::Two, SAMPLE), 195);
        assert_eq!(do_it(Part::Two, INPUT), 216);
    }
}
