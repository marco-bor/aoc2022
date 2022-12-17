use std::str::FromStr;

use pathfinding::prelude::bfs;

fn main() {
    let input = include_str!("input.txt");

    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> usize {
    let map = input.parse::<Map>().unwrap();
    bfs(&map.start, |p| map.successor(p), |p| p == &map.end)
        .expect("no path found")
        .len()
        - 1
}

fn problem2(input: &str) -> usize {
    let map = input.parse::<Map>().unwrap();

    let mut paths = map
        .heights
        .iter()
        .enumerate()
        .filter(|(_, v)| **v == 0)
        .map(|(i, _)| Pos(i % map.width, i / map.width))
        .flat_map(|pos| bfs(&pos, |p| map.successor(p), |p| p == &map.end).map(|it| it.len() - 1))
        .collect::<Vec<_>>();
    paths.sort();
    paths[0]
}

#[test]
fn test_problem1() {
    assert_eq!(problem1(include_str!("testdata.txt")), 31);
}

#[test]
fn test_problem2() {
    assert_eq!(problem2(include_str!("testdata.txt")), 29);
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn to_usize(&self, width: usize) -> usize {
        self.0 + width * self.1
    }
}

#[derive(Debug)]
struct Map {
    start: Pos,
    end: Pos,
    width: usize,
    heights: Vec<u8>,
}

impl Map {
    fn get_value(&self, pos: &Pos) -> Option<u8> {
        let index = pos.to_usize(self.width);

        if index < self.heights.len() {
            Some(self.heights[index])
        } else {
            None
        }
    }

    fn successor(&self, pos: &Pos) -> Vec<Pos> {
        let &Pos(x, y) = pos;

        let mut successors: Vec<Pos> = vec![];

        if self.can_go_right(pos) {
            successors.push(Pos(x + 1, y))
        }

        if self.can_go_left(pos) {
            successors.push(Pos(x - 1, y))
        }

        if self.can_go_down(pos) {
            successors.push(Pos(x, y + 1))
        }

        if self.can_go_up(pos) {
            successors.push(Pos(x, y - 1))
        }

        successors
    }

    fn can_go_right(&self, pos: &Pos) -> bool {
        let &Pos(x, y) = pos;
        if x + 1 == self.width {
            return false;
        }
        let (Some(current), Some(next)) = (self.get_value(pos), self.get_value(&Pos(x + 1, y))) else {
            return false;
        };

        next <= current + 1
    }

    fn can_go_left(&self, pos: &Pos) -> bool {
        let &Pos(x, y) = pos;
        if x == 0 {
            return false;
        }
        let (Some(current), Some(next)) = (self.get_value(pos), self.get_value(&Pos(x - 1, y))) else {
            return false;
        };

        next <= current + 1
    }

    fn can_go_down(&self, pos: &Pos) -> bool {
        let &Pos(x, y) = pos;
        if y + 1 == self.heights.len() / self.width {
            return false;
        }
        let (Some(current), Some(next)) = (self.get_value(pos), self.get_value(&Pos(x, y + 1))) else {
            return false;
        };

        next <= current + 1
    }

    fn can_go_up(&self, pos: &Pos) -> bool {
        let &Pos(x, y) = pos;
        if y == 0 {
            return false;
        }
        let (Some(current), Some(next)) = (self.get_value(pos), self.get_value(&Pos(x, y - 1))) else {
            return false;
        };

        next <= current + 1
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = Pos(0, 0);
        let mut end = Pos(0, 0);
        let width = s.lines().next().unwrap().len();
        let mut heights = vec![];

        for c in s.chars() {
            match c {
                'S' => {
                    start = Pos(heights.len() % width, heights.len() / width);
                    heights.push(char_to_num('a'));
                }
                'E' => {
                    end = Pos(heights.len() % width, heights.len() / width);
                    heights.push(char_to_num('z'))
                }
                'a'..='z' => heights.push(char_to_num(c)),
                _ => {}
            }
        }

        Ok(Self {
            start,
            end,
            width,
            heights,
        })
    }
}

const fn char_to_num(c: char) -> u8 {
    c as u8 - b'a'
}

#[test]
fn test_can_go_down() {
    let map = include_str!("testdata.txt").parse::<Map>().unwrap();
    assert!(map.can_go_down(&Pos(0, 0)));
    assert!(map.can_go_down(&Pos(1, 1)));
    assert!(!map.can_go_down(&Pos(4, 0)));
}

#[test]
fn test_can_go_up() {
    let map = include_str!("testdata.txt").parse::<Map>().unwrap();
    assert!(!map.can_go_up(&Pos(0, 0)));
    assert!(map.can_go_up(&Pos(6, 3)));
}

#[test]
fn test_can_go_right() {
    let map = include_str!("testdata.txt").parse::<Map>().unwrap();
    assert!(map.can_go_right(&Pos(0, 0)));
    assert!(map.can_go_right(&Pos(5, 3)));
    assert!(!map.can_go_right(&Pos(7, 0)));
}

#[test]
fn test_can_go_left() {
    let map = include_str!("testdata.txt").parse::<Map>().unwrap();
    assert!(!map.can_go_left(&Pos(0, 0)));
    assert!(map.can_go_left(&Pos(6, 1)));
}
