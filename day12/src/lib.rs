use std::collections::{HashMap, HashSet, VecDeque};

#[allow(dead_code)]
enum Part {
    One,
    Two,
}

#[allow(dead_code)]
fn do_it(part: Part, content: &str) -> usize {
    let mut edges: HashMap<String, HashSet<String>> = HashMap::new();
    for line in content.lines() {
        let parts: Vec<String> = line.split('-').map(|s| s.to_string()).collect();
        let a = &parts[0];
        let b = &parts[1];
        edges
            .entry(a.to_string())
            .or_insert_with(|| HashSet::from([b.to_string()]))
            .insert(b.to_string());
        edges
            .entry(b.to_string())
            .or_insert_with(|| HashSet::from([a.to_string()]))
            .insert(a.to_string());
    }

    let start: String = "start".to_string();
    let end: String = "end".to_string();
    let mut paths = 0;

    let mut queue: VecDeque<(String, HashSet<String>, bool)> =
        VecDeque::from([(start.clone(), HashSet::from([start.clone()]), false)]);

    while !queue.is_empty() {
        let (node, small_caves, visited_twice) = queue.pop_front().unwrap();

        if node == end {
            paths += 1;
            continue;
        }

        for adjacent_node in &edges[&node] {
            if !small_caves.contains(adjacent_node) {
                let mut updated_small_caves = small_caves.clone();
                if adjacent_node.to_lowercase() == *adjacent_node {
                    updated_small_caves.insert(adjacent_node.to_string());
                }
                queue.push_front((adjacent_node.clone(), updated_small_caves, visited_twice));
                continue;
            }
            if let Part::Two = part {
                if small_caves.contains(adjacent_node)
                    && !visited_twice
                    && !HashSet::from([start.clone(), end.clone()]).contains(adjacent_node)
                {
                    queue.push_front((adjacent_node.clone(), small_caves.clone(), true));
                }
            }
        }
    }

    paths
}

#[cfg(test)] // cargo test -- --show-output [TEST_NAME]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("data/sample.txt");
    const INPUT: &str = include_str!("data/input.txt");

    #[test]
    fn part_1() {
        assert_eq!(do_it(Part::One, SAMPLE), 10);
        assert_eq!(do_it(Part::One, INPUT), 5874);
    }

    #[test]
    fn part_2() {
        assert_eq!(do_it(Part::Two, SAMPLE), 36);
        assert_eq!(do_it(Part::Two, INPUT), 153592);
    }
}
