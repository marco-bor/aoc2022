use std::{collections::HashSet, str::FromStr};

fn main() {
    let input = include_str!("input.txt");

    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> usize {
    let mut cave: Cave = input.parse().unwrap();
    let initial_rocks = cave.rocks.len();
    let max_rock_y = *cave.rocks.iter().map(|Pos(_, y)| y).max().unwrap();

    loop {
        cave = cave.step();
        if cave.current.1 == max_rock_y {
            break;
        }
    }

    cave.rocks.len() - initial_rocks
}

fn problem2(input: &str) -> usize {
    let mut cave: Cave = input.parse().unwrap();
    let initial_rocks = cave.rocks.len();
    let max_rock_y = *cave.rocks.iter().map(|Pos(_, y)| y).max().unwrap();
    loop {
        let was_source = cave.current == SOURCE;
        let new = cave.step2(max_rock_y + 2);
        if was_source && new.current == SOURCE {
            return new.rocks.len() - initial_rocks;
        }
        cave = new;
    }
}

#[test]
fn test_problem1() {
    assert_eq!(problem1(include_str!("testdata.txt")), 24);
}

#[test]
fn test_problem2() {
    assert_eq!(problem2(include_str!("testdata.txt")), 93);
}

const SOURCE: Pos = Pos(500, 0);

#[derive(Debug)]
struct Cave {
    rocks: HashSet<Pos>,
    current: Pos,
}

impl Cave {
    fn step(self) -> Self {
        let Pos(x, y) = self.current;
        let (down, down_left, down_right) = (Pos(x, y + 1), Pos(x - 1, y + 1), Pos(x + 1, y + 1));

        let mut current = self.current;
        let mut rocks = self.rocks;

        if !rocks.contains(&down) {
            current = down;
        } else if !rocks.contains(&down_left) {
            current = down_left;
        } else if !rocks.contains(&down_right) {
            current = down_right;
        } else {
            rocks.insert(current);
            current = SOURCE.clone();
        }

        Self { rocks, current }
    }

    fn step2(self, max: usize) -> Self {
        let Pos(x, y) = self.current;
        let (down, down_left, down_right) = (Pos(x, y + 1), Pos(x - 1, y + 1), Pos(x + 1, y + 1));

        let mut current = self.current;
        let mut rocks = self.rocks;

        if y + 1 == max {
            rocks.insert(current);
            current = SOURCE.clone();
        } else if !rocks.contains(&down) {
            current = down;
        } else if !rocks.contains(&down_left) {
            current = down_left;
        } else if !rocks.contains(&down_right) {
            current = down_right;
        } else {
            rocks.insert(current);
            current = SOURCE.clone();
        }

        Self { rocks, current }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos(usize, usize);

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rocks = s
            .lines()
            .flat_map(|l| {
                let edges = l
                    .split(" -> ")
                    .flat_map(|s| s.parse::<Pos>())
                    .collect::<Vec<_>>();
                edges
                    .windows(2)
                    .flat_map(|chunks| match chunks {
                        [Pos(x1, y1), Pos(x2, y2)] if x1 == x2 => {
                            let (min, max) = (*y1.min(y2), *y1.max(y2));
                            (min..=max).map(|y| Pos(*x1, y)).collect::<Vec<_>>()
                        }
                        [Pos(x1, y1), Pos(x2, y2)] if y1 == y2 => {
                            let (min, max) = (*x1.min(x2), *x1.max(x2));
                            (min..=max).map(|x| Pos(x, *y1)).collect::<Vec<_>>()
                        }
                        _ => unreachable!("wtf"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<HashSet<_>>();

        Ok(Self {
            rocks,
            current: SOURCE.clone(),
        })
    }
}

impl FromStr for Pos {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        Ok(Pos(x.parse().unwrap(), y.parse().unwrap()))
    }
}
