use std::{fmt::Debug, str::FromStr, vec};

fn main() {
    let input = include_str!("input.txt");

    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> usize {
    let pairs = parse_node_pairs(input);
    pairs
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| left <= right)
        .map(|(i, _)| i + 1)
        .sum()
}

fn problem2(input: &str) -> usize {
    let mut nodes = input
        .lines()
        .flat_map(|l| l.parse::<Node>())
        .collect::<Vec<_>>();

    let divider2: Node = "[[2]]".parse().unwrap();
    nodes.push(divider2.clone());
    let divider6: Node = "[[6]]".parse().unwrap();
    nodes.push(divider6.clone());

    nodes.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let divider2_idx = 1 + nodes
        .binary_search_by(|n| n.partial_cmp(&divider2).unwrap())
        .unwrap();
    let divider6_idx = 1 + nodes
        .binary_search_by(|n| n.partial_cmp(&divider6).unwrap())
        .unwrap();
    divider2_idx * divider6_idx
}

#[test]
fn test_problem1() {
    assert_eq!(problem1(include_str!("testdata.txt")), 13);
}

#[test]
fn test_problem2() {
    assert_eq!(problem2(include_str!("testdata.txt")), 140);
}

#[derive(Debug, PartialEq, Clone)]
enum Node {
    Simple(u8),
    Complex(Vec<Node>),
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Node::Simple(left), Node::Simple(right)) => left.partial_cmp(right),
            (Node::Simple(_), Node::Complex(_)) => {
                Self::Complex(vec![self.clone()]).partial_cmp(other)
            }
            (Node::Complex(_), Node::Simple(_)) => {
                self.partial_cmp(&Self::Complex(vec![other.clone()]))
            }
            (Node::Complex(left), Node::Complex(right)) => {
                for i in 0..usize::min(left.len(), right.len()) {
                    let cmp = left[i].partial_cmp(&right[i]).unwrap();
                    if cmp.is_ne() {
                        return Some(cmp);
                    }
                }

                left.len().partial_cmp(&right.len())
            }
        }
    }
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack: Vec<Node> = Vec::default();
        let mut chars = s.chars().peekable();

        loop {
            let Some(c) = chars.next() else {
                break;
            };

            match c {
                '0'..='9' => {
                    let mut num = vec![c];
                    while let Some(_c @ '0'..='9') = chars.peek() {
                        num.push(chars.next().unwrap())
                    }

                    let a = num
                        .iter()
                        .collect::<String>()
                        .parse()
                        .expect("could not parse number");

                    if let Some(Self::Complex(v)) = stack.last_mut() {
                        v.push(Self::Simple(a))
                    }
                }
                '[' => stack.push(Self::Complex(Vec::default())),
                ']' => {
                    if stack.len() > 1 {
                        let node = stack.pop().unwrap();
                        if let Some(Self::Complex(v)) = stack.last_mut() {
                            v.push(node)
                        }
                    }
                }
                _ => {}
            }
        }

        stack.pop().ok_or(())
    }
}

fn parse_node_pairs(s: &str) -> Vec<(Node, Node)> {
    s.split("\n\n")
        .flat_map(|l| l.split_once('\n'))
        .flat_map(|(left, right)| {
            let (Ok(left), Ok(right)) = (left.parse::<Node>(), right.parse::<Node>()) else {
            return None;
        };
            Some((left, right))
        })
        .collect::<Vec<_>>()
}
