use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Node {
    label: String,
    left: String,
    right: String,
}

fn parse_node(line: &str) -> Node {
    let label = line[..3].to_string();
    let left = line[7..10].to_string();
    let right = line[12..15].to_string();
    Node { label, left, right }
}

#[derive(Debug)]
struct Graph {
    left: Vec<usize>,
    right: Vec<usize>,
    start: usize,
    end: usize,
}

impl Graph {
    fn new(nodes: &[Node]) -> Self {
        let mut mapping = HashMap::new();
        for (i, node) in nodes.iter().enumerate() {
            mapping.insert(node.label.clone(), i);
        }
        let left: Vec<usize> = nodes
            .iter()
            .map(|n| *mapping.get(&n.left).unwrap())
            .collect();
        let right: Vec<usize> = nodes
            .iter()
            .map(|n| *mapping.get(&n.right).unwrap())
            .collect();
        let start = *mapping.get("AAA").unwrap();
        let end = *mapping.get("ZZZ").unwrap();
        Self {
            left,
            right,
            start,
            end,
        }
    }
}

#[derive(Debug)]
struct GhostGraph {
    left: Vec<usize>,
    right: Vec<usize>,
    starts: Vec<usize>,
    ends: HashSet<usize>,
}

impl GhostGraph {
    fn new(nodes: &[Node]) -> Self {
        let mut mapping = HashMap::new();
        for (i, node) in nodes.iter().enumerate() {
            mapping.insert(node.label.clone(), i);
        }
        let left: Vec<usize> = nodes
            .iter()
            .map(|n| *mapping.get(&n.left).unwrap())
            .collect();
        let right: Vec<usize> = nodes
            .iter()
            .map(|n| *mapping.get(&n.right).unwrap())
            .collect();
        let starts: Vec<usize> = nodes
            .iter()
            .filter(|n| n.label.ends_with('A'))
            .map(|n| *mapping.get(&n.label).unwrap())
            .collect();
        let ends: HashSet<usize> = nodes
            .iter()
            .filter(|n| n.label.ends_with('Z'))
            .map(|n| *mapping.get(&n.label).unwrap())
            .collect();
        Self {
            left,
            right,
            starts,
            ends,
        }
    }
}

fn part1(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let dirs = lines[0].as_bytes();
    let nodes: Vec<Node> = lines[2..].iter().map(|l| parse_node(l)).collect();
    let graph = Graph::new(&nodes);
    let mut dist = 0;
    let mut curr = graph.start;
    while curr != graph.end {
        let dir = dirs[dist % dirs.len()] as char;
        if dir == 'L' {
            curr = graph.left[curr];
        } else {
            curr = graph.right[curr];
        }
        dist += 1;
    }
    dist as u32
}

fn part2(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let dirs = lines[0].as_bytes();
    let nodes: Vec<Node> = lines[2..].iter().map(|l| parse_node(l)).collect();
    let graph = GhostGraph::new(&nodes);
    let dists: Vec<u64> = graph
        .starts
        .iter()
        .map(|start| {
            let mut dist = 0;
            let mut curr = *start;
            while !graph.ends.contains(&curr) {
                let dir = dirs[dist % dirs.len()] as char;
                if dir == 'L' {
                    curr = graph.left[curr];
                } else {
                    curr = graph.right[curr];
                }
                dist += 1;
            }
            dist as u64
        })
        .collect();
    dists.iter().fold(1, |a, &b| a * b / gcd::euclid_u64(a, b))
}

fn main() {
    let input = include_str!("input/8.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}
