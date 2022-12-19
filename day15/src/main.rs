use std::{collections::HashSet, str::FromStr};

use rayon::prelude::*;

fn main() {
    let input = include_str!("input.txt");

    println!("Problem 1: {}", problem1(input, 2_000_000));
    println!("Problem 2: {}", problem2(input, 4_000_000));
}

fn problem1(input: &str, y: i32) -> usize {
    let measures = parse_measures(input);
    let mut visited = visited_pos(measures.as_slice(),y);
    measures
        .iter()
        .flat_map(|m| [&m.closest, &m.sensor])
        .for_each(|pos| {
            visited.remove(pos);
        });

    visited.len()
}

fn visited_pos(measures: &[Measure], y: i32) -> HashSet<Pos> {
    measures
        .iter()
        .filter(|m| (m.sensor.1.abs_diff(y) as usize) <= m.dist())
        .flat_map(|m| {
            let dist_y = (m.dist() - m.sensor.1.abs_diff(y) as usize) as i32;
            (0..=dist_y).flat_map(|d| [Pos(m.sensor.0 + d, y), Pos(m.sensor.0 - d, y)])
        })
        .collect::<HashSet<_>>()
}

fn problem2(input: &str, max: usize) -> u32 {
    let measures = parse_measures(input);
    0
}

#[test]
fn test_problem1() {
    assert_eq!(problem1(include_str!("testdata.txt"), 10), 26);
}

#[test]
fn test_problem2() {
    assert_eq!(problem2(include_str!("testdata.txt"), 20), 0);
}

#[derive(Debug, PartialEq)]
struct Measure {
    sensor: Pos,
    closest: Pos,
}
impl Measure {
    fn dist(&self) -> usize {
        self.sensor.dist(&self.closest)
    }
}

fn parse_measures(s: &str) -> Vec<Measure> {
    s.lines().flat_map(|s| s.parse()).collect::<Vec<_>>()
}

impl FromStr for Measure {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [sensor_x, sensor_y, beacon_x, beacon_y] = s
            .split(&[',', ' ', ':'])
            .filter(|s| s.contains('='))
            .map(|s| s.split_once('=').unwrap().1)
            .collect::<Vec<_>>()[..] else {
                return Err(());
            };

        let (Ok(sensor_x), Ok(sensor_y), Ok(beacon_x), Ok(beacon_y)) = (sensor_x.parse(),sensor_y.parse(),beacon_x.parse(),beacon_y.parse()) else{
            return Err(());
        };

        Ok(Self {
            sensor: Pos(sensor_x, sensor_y),
            closest: Pos(beacon_x, beacon_y),
        })
    }
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct Pos(i32, i32);

impl Pos {
    fn dist(&self, other: &Pos) -> usize {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as usize
    }
}

#[test]
fn test_dist() {
    assert_eq!(Pos(8, 7).dist(&Pos(2, 10)), 9);
}

#[test]
fn test_parse() {
    assert_eq!(
        "Sensor at x=3482210, y=422224: closest beacon is at x=2273934, y=-202439".parse(),
        Ok(Measure {
            sensor: Pos(3482210, 422224),
            closest: Pos(2273934, -202439)
        })
    );
}
