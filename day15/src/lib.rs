// Most of this solution is pulled directly from:
// https://doc.rust-lang.org/std/collections/binary_heap/index.html

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[allow(dead_code)]
enum Part {
    One,
    Two,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Edge {
    node: usize,
    cost: usize,
}

fn shortest_path(adj_list: &[Vec<Edge>], start: usize, goal: usize) -> Option<usize> {
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }
        if cost > dist[position] {
            continue;
        }

        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }

    None
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn grid_to_graph(grid: Vec<Vec<usize>>) -> Vec<Vec<Edge>> {
    let mut graph: Vec<Vec<Edge>> = Vec::new();
    let mut dirs: Vec<Dir> = Vec::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let mut node: Vec<Edge> = Vec::new();
            let col_len = grid.len();
            let row_len = grid[0].len();

            let node_id = |a, b| a * row_len + b;

            if i == 0 && j == 0 {
                // upper left corner
                dirs = Vec::from([Dir::Down, Dir::Right]);
            } else if i == 0 && j > 0 && j < row_len - 1 {
                // top row
                dirs = Vec::from([Dir::Down, Dir::Left, Dir::Right]);
            } else if i == 0 && j == row_len - 1 {
                // upper right corner
                dirs = Vec::from([Dir::Down, Dir::Left]);
            } else if i > 0 && i < col_len - 1 && j == 0 {
                // left edge
                dirs = Vec::from([Dir::Up, Dir::Down, Dir::Right]);
            } else if i > 0 && i < col_len - 1 && j > 0 && j < row_len - 1 {
                // interior
                dirs = Vec::from([Dir::Up, Dir::Down, Dir::Left, Dir::Right]);
            } else if i > 0 && i < col_len - 1 && j == row_len - 1 {
                // right edge
                dirs = Vec::from([Dir::Up, Dir::Down, Dir::Left]);
            } else if i == col_len - 1 && j == 0 {
                // lower left corner
                dirs = Vec::from([Dir::Up, Dir::Right]);
            } else if i == col_len - 1 && j > 0 && j < row_len - 1 {
                // bottom row
                dirs = Vec::from([Dir::Up, Dir::Left, Dir::Right]);
            } else if i == col_len - 1 && j == row_len - 1 {
                // lower right corner
                dirs = Vec::from([Dir::Up, Dir::Left]);
            }

            for dir in &dirs {
                match dir {
                    Dir::Up => node.push(Edge {
                        node: node_id(i - 1, j),
                        cost: grid[i - 1][j],
                    }),
                    Dir::Down => node.push(Edge {
                        node: node_id(i + 1, j),
                        cost: grid[i + 1][j],
                    }),
                    Dir::Left => node.push(Edge {
                        node: node_id(i, j - 1),
                        cost: grid[i][j - 1],
                    }),
                    Dir::Right => node.push(Edge {
                        node: node_id(i, j + 1),
                        cost: grid[i][j + 1],
                    }),
                }
            }

            dirs.clear();
            graph.push(node);
        }
    }

    graph
}

#[allow(dead_code)]
fn do_it(part: Part, content: &str) -> usize {
    let mut grid: Vec<Vec<usize>> = content
        .lines()
        .map(|l| {
            l.chars()
                .map(|v| v.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let mut row_len = grid.len();
    let mut col_len = grid[0].len();
    let mut end = row_len * col_len - 1;

    if let Part::Two = part {
        let mut new_grid: Vec<Vec<usize>> = vec![vec![0; col_len * 5]; row_len * 5];
        for i in 0..row_len {
            for j in 0..col_len {
                for k in 0..5 {
                    for l in 0..5 {
                        let val = match grid[i][j] + k + l <= 9 {
                            true => grid[i][j] + k + l,
                            false => (grid[i][j] + k + l) - 9,
                        };
                        new_grid[k * row_len + i][l * col_len + j] = val;
                    }
                }
            }
        }
        grid = new_grid;
        row_len = grid.len();
        col_len = grid[0].len();
        end = row_len * col_len - 1;
    }

    let graph: Vec<Vec<Edge>> = grid_to_graph(grid);

    shortest_path(&graph, 0, end).unwrap()
}

#[cfg(test)] // cargo test -- --show-output [TEST_NAME]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("data/sample.txt");
    const INPUT: &str = include_str!("data/input.txt");

    #[test]
    fn part_1() {
        assert_eq!(do_it(Part::One, SAMPLE), 40);
        assert_eq!(do_it(Part::One, INPUT), 824);
    }

    #[test]
    fn part_2() {
        assert_eq!(do_it(Part::Two, SAMPLE), 315);
        assert_eq!(do_it(Part::Two, INPUT), 3063);
    }
}
