use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use num::integer::lcm;
use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(ch: char) -> Self {
        match ch {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid character, only 'L' or 'R' allowed. Found '{}'", ch),
        }
    }
}

struct Node {
    name: String,
    left_ref: String,
    right_ref: String,
}

impl Node {
    fn next_node(&self, direction: &Direction) -> &str {
        match direction {
            Direction::Left => &self.left_ref,
            Direction::Right => &self.right_ref,
        }
    }

    fn is_start_node(&self) -> bool {
        self.name.ends_with("A")
    }

    fn is_end_node(&self) -> bool {
        self.name.ends_with("Z")
    }
}

impl From<&str> for Node {
    fn from(s: &str) -> Self {
        // example: "BBB = (DDD, EEE)"
        let (name, directions) = s.split_once(" = ").unwrap();

        Node {
            name: name.to_string(),
            left_ref: directions[1..4].to_string(),
            right_ref: directions[6..9].to_string(),
        }
    }
}

struct PuzzleData {
    directions: Vec<Direction>,
    node_map: HashMap<String, Node>,
}

impl PuzzleData {
    fn new(input: &str) -> PuzzleData {
        let (directions_str, network_str) = input.split_once("\n\n").unwrap();
        let directions = directions_str.chars().map(Direction::from).collect_vec();
        let nodes = network_str.split("\n").map_into::<Node>().collect_vec();
        let node_map = nodes.into_iter().fold(HashMap::new(), |mut map, row| {
            map.insert(row.name.clone(), row);
            map
        });
        PuzzleData {
            directions,
            node_map,
        }
    }
}

fn find_end_node_idx(start_node: &Node, puzzle_data: &PuzzleData) -> i64 {
    let directions = &puzzle_data.directions;
    let node_map = &puzzle_data.node_map;
    let result =
        directions
            .iter()
            .cycle()
            .fold_while((start_node, 0), |(current_node, idx), direction| {
                if current_node.is_end_node() {
                    Done((current_node, idx))
                } else {
                    let next_node = node_map.get(current_node.next_node(direction)).unwrap();
                    Continue((next_node, idx + 1))
                }
            });
    match result {
        Done((_end_nodes, idx)) => idx,
        _ => panic!("Unexpected state!"),
    }
}

pub fn part_a(input: &str) -> u32 {
    let puzzle_data = PuzzleData::new(input);
    let directions = puzzle_data.directions;
    let result = directions
        .iter()
        .cycle()
        .fold_while(("AAA", 0), |(node_name, idx), direction| {
            if node_name == "ZZZ" {
                Done((node_name, idx))
            } else {
                let next_node = puzzle_data
                    .node_map
                    .get(node_name)
                    .unwrap()
                    .next_node(direction);
                Continue((next_node, idx + 1))
            }
        });
    match result {
        Done((_node_name, idx)) => idx,
        _ => panic!("Unexpected state!"),
    }
}

pub fn part_b(input: &str) -> i64 {
    let puzzle_data = PuzzleData::new(input);
    let starting_nodes = puzzle_data
        .node_map
        .values()
        .filter(|&v| v.is_start_node())
        .collect_vec();
    let end_node_idxs = starting_nodes
        .iter()
        .map(|&n| find_end_node_idx(n, &puzzle_data))
        .collect_vec();

    // find the lowest common multiple of each of the end states
    end_node_idxs.iter().fold(1, |acc, &x| lcm(acc, x))
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2023::read_test_file;

    #[test]
    fn test_part_a() {
        let input = read_test_file(8);
        assert_eq!(part_a(&input), 2);
    }

    #[test]
    fn test_part_b() {
        let input = read_test_file(8);
        assert_eq!(part_b(&input), 2);
    }
}
