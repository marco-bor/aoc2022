use std::{str::FromStr, vec};

fn main() {
    let input = include_str!("input.txt");

    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> String {
    let (stacks, rearrangement) = input
        .split_once("\n\n")
        .expect("invalid input. expected double newline");

    let mut stacks = load_stacks(stacks);

    for r in rearrangement
        .lines()
        .flat_map(|l| l.parse::<Rearrangement>())
    {
        for _ in 0..(r.qty) {
            let char = stacks[(r.from - 1) as usize]
                .pop()
                .expect("stack is empty. cannot move any more items");
            stacks[(r.to - 1) as usize].push(char);
        }
    }

    stacks.iter_mut().flat_map(|s| s.pop()).collect()
}

#[test]
fn test_problem1() {
    assert_eq!(problem1(include_str!("testdata.txt")), "CMZ");
}

fn problem2(input: &str) -> String {
    let (stacks, rearrangement) = input
        .split_once("\n\n")
        .expect("invalid input. expected double newline");

    let mut stacks = load_stacks(stacks);

    let mut temp_stack: Vec<char> = Vec::default();

    for r in rearrangement
        .lines()
        .flat_map(|l| l.parse::<Rearrangement>())
    {
        for _ in 0..(r.qty) {
            let char = stacks[(r.from - 1) as usize]
                .pop()
                .expect("stack is empty. cannot move any more items");
            temp_stack.push(char)
        }

        for _ in 0..(r.qty) {
            stacks[(r.to - 1) as usize].push(temp_stack.pop().expect("temp_stack is empty. cannot pop items"));
        }
    }

    stacks.iter_mut().flat_map(|s| s.pop()).collect()
}

#[test]
fn test_problem2() {
    assert_eq!(problem2(include_str!("testdata.txt")), "MCD");
}

struct Rearrangement {
    qty: u32,
    from: u32,
    to: u32,
}

impl FromStr for Rearrangement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split_whitespace().flat_map(|l| l.parse::<u32>());

        Ok(Rearrangement {
            qty: nums.next().unwrap(),
            from: nums.next().unwrap(),
            to: nums.next().unwrap(),
        })
    }
}

fn load_stacks(stacks: &str) -> Vec<Vec<char>> {
    let nstacks = stacks.lines().last().unwrap().split_whitespace().count();
    let mut vec_stacks: Vec<Vec<char>> = vec![Vec::default(); nstacks];

    for (i, vec) in vec_stacks.iter_mut().enumerate() {
        let char_idx = 1 + i * 4;

        for char in stacks
            .lines()
            .rev()
            .skip(1)
            .map(|l| l.as_bytes()[char_idx] as char)
        {
            if char != ' ' {
                vec.push(char)
            }
        }
    }

    vec_stacks
}

#[test]
fn test_load_stacks() {
    let (stacks, _) = include_str!("testdata.txt")
        .split_once("\n\n")
        .expect("invalid input. expected double newline");

    let mut stacks = load_stacks(stacks);

    assert_eq!(stacks[0].pop(), Some('N'));
    assert_eq!(stacks[0].pop(), Some('Z'));
    assert_eq!(stacks[0].pop(), None);

    assert_eq!(stacks[1].pop(), Some('D'));
    assert_eq!(stacks[1].pop(), Some('C'));
    assert_eq!(stacks[1].pop(), Some('M'));
    assert_eq!(stacks[1].pop(), None);

    assert_eq!(stacks[2].pop(), Some('P'));
    assert_eq!(stacks[2].pop(), None);
}
