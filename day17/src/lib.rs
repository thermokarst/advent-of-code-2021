use std::cmp::Ordering;

#[allow(dead_code)]
enum Part {
    One,
    Two,
}

#[derive(Debug)]
struct Projectile {
    x_pos: isize,
    y_pos: isize,
    x_vel: isize,
    y_vel: isize,
    max_y: isize,
}

impl Projectile {
    fn tick(&mut self) {
        self.x_pos += self.x_vel;
        self.y_pos += self.y_vel;

        if self.y_pos > self.max_y {
            self.max_y = self.y_pos;
        }

        match self.x_vel.cmp(&0) {
            Ordering::Greater => self.x_vel -= 1,
            Ordering::Less => self.x_vel += 1,
            Ordering::Equal => self.x_vel = 0,
        }

        self.y_vel -= 1;
    }

    fn in_target(&self, target: &BoxCoords) -> bool {
        let (left, right, bottom, top) = target;
        self.x_pos >= *left && self.x_pos <= *right && self.y_pos >= *bottom && self.y_pos <= *top
    }

    fn past_target(&self, target: &BoxCoords) -> bool {
        let (_, right, bottom, _) = target;
        self.x_pos > *right || self.y_pos < *bottom
    }
}

type BoxCoords = (isize, isize, isize, isize);

#[allow(dead_code)]
fn do_it(part: Part, data: BoxCoords) -> isize {
    let mut max_y = -100000;
    let mut count = 0;

    for x_vel in -300..300 {
        for y_vel in -300..300 {
            let mut projectile = Projectile {
                x_pos: 0,
                y_pos: 0,
                x_vel,
                y_vel,
                max_y,
            };
            'steps: for _step in 1..300 {
                projectile.tick();

                if projectile.past_target(&data) {
                    break 'steps;
                }

                if projectile.in_target(&data) {
                    count += 1;

                    if projectile.max_y > max_y {
                        max_y = projectile.max_y;
                    }
                    break 'steps;
                }
            }
        }
    }

    match part {
        Part::One => max_y,
        Part::Two => count,
    }
}

#[cfg(test)] // cargo test -- --show-output [TEST_NAME]
mod tests {
    use super::*;

    const SAMPLE: BoxCoords = (20, 30, -10, -5);
    const INPUT: BoxCoords = (217, 240, -126, -69);

    #[test]
    fn part_1() {
        assert_eq!(do_it(Part::One, SAMPLE), 45);
        assert_eq!(do_it(Part::One, INPUT), 7875);
    }

    #[test]
    fn part_2() {
        assert_eq!(do_it(Part::Two, SAMPLE), 112);
        assert_eq!(do_it(Part::Two, INPUT), 2321);
    }
}
