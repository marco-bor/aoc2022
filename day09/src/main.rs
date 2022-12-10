use std::str::FromStr;

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");

    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> usize {
    let movements = parse_movements(input);
    tail_positions(movements).iter().unique().count()
}

fn problem2(input: &str) -> usize {
    let movements = parse_movements(input);
    tail_positions(movements).iter().skip(9).unique().count()
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    /// dist = max(|x1-x2|, |y1-y2|)
    fn dist(&self, other: &Pos) -> u32 {
        let x = self.x.abs_diff(other.x);
        let y = self.y.abs_diff(other.y);

        x.max(y)
    }
}

fn tail_positions(movements: Vec<Movement>) -> Vec<Pos> {
    let mut head = Pos { x: 1, y: 1 };
    let mut tail = Pos { x: 1, y: 1 };

    let mut tails = vec![tail.clone()];

    for m in movements {
        match m {
            Movement::Up(delta) => {
                for _ in 0..delta {
                    head.y += 1;

                    if tail.dist(&head) > 1 {
                        tail.y += 1;
                        tail.x = head.x;
                        tails.push(tail.clone());
                    }
                }
            }
            Movement::Down(delta) => {
                for _ in 0..delta {
                    head.y -= 1;

                    if tail.dist(&head) > 1 {
                        tail.y -= 1;
                        tail.x = head.x;
                        tails.push(tail.clone());
                    }
                }
            }
            Movement::Right(delta) => {
                for _ in 0..delta {
                    head.x += 1;

                    if tail.dist(&head) > 1 {
                        tail.x += 1;
                        tail.y = head.y;
                        tails.push(tail.clone());
                    }
                }
            }
            Movement::Left(delta) => {
                for _ in 0..delta {
                    head.x -= 1;

                    if tail.dist(&head) > 1 {
                        tail.x -= 1;
                        tail.y = head.y;
                        tails.push(tail.clone());
                    }
                }
            }
        }
    }

    tails
}

enum Movement {
    Up(u8),
    Down(u8),
    Left(u8),
    Right(u8),
}

impl FromStr for Movement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(' ');
        match tokens.next().unwrap() {
            "L" => Ok(Self::Left(tokens.next().unwrap().parse::<u8>().unwrap())),
            "R" => Ok(Self::Right(tokens.next().unwrap().parse::<u8>().unwrap())),
            "D" => Ok(Self::Down(tokens.next().unwrap().parse::<u8>().unwrap())),
            "U" => Ok(Self::Up(tokens.next().unwrap().parse::<u8>().unwrap())),
            _ => Err(()),
        }
    }
}

fn parse_movements(input: &str) -> Vec<Movement> {
    input
        .lines()
        .flat_map(|l| l.parse::<Movement>())
        .collect::<Vec<_>>()
}

#[test]
fn test_problem1() {
    assert_eq!(problem1(include_str!("testdata.txt")), 13);
}

#[test]
fn test_problem2() {
    assert_eq!(problem2(include_str!("testdata2.txt")), 36);
}
