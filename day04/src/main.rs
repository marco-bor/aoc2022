use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");

    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> u32 {
    input
        .lines()
        .flat_map(|l| l.split_once(','))
        .flat_map(|r| {
            let (Ok(a),Ok(b)) = (r.0.parse::<Range>(), r.1.parse::<Range>()) else {
                return None
            };
            Some((a, b))
        })
        .filter(|(a, b)| a.contains(&b) || b.contains(&a))
        .count() as u32
}

fn problem2(input: &str) -> u32 {
    input
        .lines()
        .flat_map(|l| l.split_once(','))
        .flat_map(|r| {
            let (Ok(a),Ok(b)) = (r.0.parse::<Range>(), r.1.parse::<Range>()) else {
                return None
            };
            Some((a, b))
        })
        .filter(|(a, b)| a.overlaps(&b))
        .count() as u32
}

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        !(self.start > other.end || self.end < other.start)
    }
}
impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((start, end)) = s.split_once('-') else {
            return Err(());
        };

        let (Ok(start), Ok(end)) = (start.parse::<u32>(), end.parse::<u32>()) else {
            return Err(());
        };

        Ok(Self { start, end })
    }
}
