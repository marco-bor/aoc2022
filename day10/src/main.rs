use std::{str::FromStr, vec};

fn main() {
    let input = include_str!("input.txt");

    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> i32 {
    let instr = parse_instructions(input);
    let cycles = vec![20, 60, 100, 140, 180, 220];
    cycles
        .iter()
        .map(|c| c * calc_x_at_cycle(instr.as_slice(), *c as usize))
        .sum::<i32>()
}

fn problem2(input: &str) -> u32 {
    let instr = parse_instructions(input);

    draw_crt(instr.as_slice())
        .chunks(40)
        .map(|c| c.iter().collect::<String>())
        .for_each(|s| println!("{}", s));

    0
}

#[test]
fn test_problem1() {
    assert_eq!(problem1(include_str!("testdata.txt")), 13140);
}

#[test]
fn test_problem2() {
    assert_eq!(problem2(include_str!("testdata.txt")), 0);
}

enum Instruction {
    Noop,
    Add(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(' ');
        match tokens.next().expect("expected token") {
            "noop" => Ok(Self::Noop),
            "addx" => Ok(Self::Add(
                tokens
                    .next()
                    .expect("expected token")
                    .parse::<i32>()
                    .expect("invalid token"),
            )),
            _ => Err(()),
        }
    }
}

fn calc_x_at_cycle(instructions: &[Instruction], cycle: usize) -> i32 {
    let mut x = 1;
    let mut mcycle = 1;

    for instr in instructions {
        match instr {
            Instruction::Noop => mcycle += 1,
            Instruction::Add(num) => {
                mcycle += 1;
                if cycle == mcycle {
                    break;
                }
                mcycle += 1;
                x += num;
            }
        }

        if mcycle == cycle {
            break;
        }
    }

    x
}

fn draw_crt(instructions: &[Instruction]) -> Vec<char> {
    let mut crt: Vec<char> = vec![];
    let mut x = 1;
    let mut _mcycle = 1;
    crt.push('#');

    for instr in instructions {
        match instr {
            Instruction::Noop => _mcycle += 1,
            Instruction::Add(num) => {
                _mcycle += 1;
                match (crt.len() as isize) % 40 - x as isize {
                    -1 | 0 | 1 => crt.push('#'),
                    _ => crt.push('.'),
                }
                _mcycle += 1;
                x += num;
            }
        }
        match (crt.len() as isize) % 40 - x as isize {
            -1 | 0 | 1 => crt.push('#'),
            _ => crt.push('.'),
        }
    }

    crt
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .flat_map(|l| l.parse::<Instruction>())
        .collect::<Vec<_>>()
}

#[test]
fn test_calc_x() {
    let instr = parse_instructions(include_str!("testdata.txt"));
    assert_eq!(calc_x_at_cycle(instr.as_slice(), 20), 21);
    assert_eq!(calc_x_at_cycle(instr.as_slice(), 60), 19);
    assert_eq!(calc_x_at_cycle(instr.as_slice(), 100), 18);
    assert_eq!(calc_x_at_cycle(instr.as_slice(), 140), 21);
    assert_eq!(calc_x_at_cycle(instr.as_slice(), 180), 16);
    assert_eq!(calc_x_at_cycle(instr.as_slice(), 220), 18);
}
