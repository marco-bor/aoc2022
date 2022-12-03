use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
enum What {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum RoundResult {
    Win,
    Tie,
    Loss,
}

#[derive(Debug)]
struct Round1 {
    opponent: What,
    me: What,
}

#[derive(Debug)]
struct Round2 {
    opponent: What,
    result: RoundResult,
}

impl FromStr for What {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(What::Rock),
            "B" | "Y" => Ok(What::Paper),
            "C" | "Z" => Ok(What::Scissors),
            _ => Err(()),
        }
    }
}
impl FromStr for Round1 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((first, second)) = s.split_once(' ') else {
            return Err(())
        };

        let (Ok(first), Ok(second)) = (first.parse::<What>(), second.parse::<What>()) else {
            return Err(())
        };

        Ok(Round1 {
            opponent: first,
            me: second,
        })
    }
}
impl FromStr for RoundResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Tie),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}
impl FromStr for Round2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((first, second)) = s.split_once(' ') else {
            return Err(())
        };

        let (Ok(first), Ok(second)) = (first.parse::<What>(), second.parse::<RoundResult>()) else {
            return Err(())
        };

        Ok(Round2 {
            opponent: first,
            result: second,
        })
    }
}

impl Round1 {
    fn result(&self) -> RoundResult {
        if self.opponent == self.me {
            RoundResult::Tie
        } else if match self.me {
            What::Rock => self.opponent == What::Scissors,
            What::Paper => self.opponent == What::Rock,
            What::Scissors => self.opponent == What::Paper,
        } {
            RoundResult::Win
        } else {
            RoundResult::Loss
        }
    }
}

impl Round2 {
    fn me(&self) -> What {
        match self.result {
            RoundResult::Win => match self.opponent {
                What::Rock => What::Paper,
                What::Paper => What::Scissors,
                What::Scissors => What::Rock,
            },
            RoundResult::Tie => self.opponent,
            RoundResult::Loss => match self.opponent {
                What::Rock => What::Scissors,
                What::Paper => What::Rock,
                What::Scissors => What::Paper,
            },
        }
    }
}

trait Points {
    fn points(&self) -> u32;
}

impl Points for What {
    fn points(&self) -> u32 {
        match self {
            What::Rock => 1,
            What::Paper => 2,
            What::Scissors => 3,
        }
    }
}

impl Points for RoundResult {
    fn points(&self) -> u32 {
        match self {
            RoundResult::Win => 6,
            RoundResult::Tie => 3,
            RoundResult::Loss => 0,
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input))
}

fn problem1(input: &str) -> u32 {
    input
        .lines()
        .flat_map(|s| s.parse::<Round1>())
        .map(|r| r.me.points() + r.result().points())
        .sum::<u32>()
}

fn problem2(input: &str) -> u32 {
    input
        .lines()
        .flat_map(|s| s.parse::<Round2>())
        .map(|r| r.me().points() + r.result.points())
        .sum::<u32>()
}
