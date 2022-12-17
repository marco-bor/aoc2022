use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");

    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> usize {
    let map = input.parse::<Map>().unwrap();

    map.vec
        .iter()
        .enumerate()
        .filter(|(i, _)| map.visibility(*i) > 0)
        .count()
}

fn problem2(_input: &str) -> u32 {
    0
}

struct Map {
    width: usize,
    vec: Vec<u8>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().len();
        let vec = s
            .lines()
            .flat_map(|l| l.split(""))
            .flat_map(|c| c.parse::<u8>())
            .collect::<Vec<_>>();

        Ok(Map { width, vec })
    }
}

impl Map {
    fn visibility(&self, index: usize) -> u32 {
        let mut visibility = 0u32;
        let col = index % self.width;
        let _row = (index - col) / self.width;

        if col == 0 || col == self.width - 1 {
            // first or last column
            return 1;
        }

        if index < self.width || index >= self.vec.len() - self.width {
            // first or last row
            return 1;
        }

        // if visibile from the left
        if ((index - col)..index).all(|i| self.vec[i] < self.vec[index]) {
            visibility += 1;
        }

        let next_row = index - col + self.width;

        // if visible from the right
        if ((index + 1)..next_row).all(|i| self.vec[i] < self.vec[index]) {
            visibility += 1;
        }

        // if visible from the top
        if (col..index)
            .step_by(self.width)
            .all(|i| self.vec[i] < self.vec[index])
        {
            visibility += 1;
        }

        // if visible from the bottom
        if ((index + self.width)..self.vec.len())
            .step_by(self.width)
            .all(|i| self.vec[i] < self.vec[index])
        {
            visibility += 1;
        }

        visibility
    }

    fn visibility_top(&self, index: usize) -> usize {
        let col = index % self.width;
        let row = (index - col) / self.width;

        if row == 0 {
            // first row
            return 0;
        }

        (col..index)
            .step_by(self.width)
            .rev()
            .filter(|i| self.vec[i + self.width] > self.vec[*i])
            .count()
    }

    fn visibility_bottom(&self, index: usize) -> usize {
        if index >= self.vec.len() - self.width {
            // last row
            return 0;
        }

        (index..self.vec.len())
            .step_by(self.width)
            .skip(1)
            .filter(|i| self.vec[i - self.width] > self.vec[*i])
            .count()
    }

    fn visibility_left(&self, index: usize) -> usize {
        let col = index % self.width;
        let _max = 0;

        if col == 0 {
            // first col
            return 0;
        }

        ((index - col)..index)
            .rev()
            .filter(|i| self.vec[i + 1] > self.vec[*i])
            .count()
    }

    fn visibility_right(&self, index: usize) -> usize {
        let col = index % self.width;

        if col == self.width - 1 {
            // last col
            0
        } else {
            1 + self.visibility_right(index + 1)
        }
    }
}

#[test]
fn test_problem1() {
    let input = include_str!("testdata.txt");
    assert_eq!(input.parse::<Map>().unwrap().vec.len(), 25);
    assert_eq!(problem1(input), 21);
}

#[test]
fn test_problem2() {
    assert_eq!(problem2(include_str!("testdata.txt")), 0);
}

#[test]
fn test_visibility() {
    let map = include_str!("testdata.txt").parse::<Map>().unwrap();

    assert_eq!(map.visibility(0), 1);
    assert_eq!(map.visibility(4), 1);
    assert_eq!(map.visibility(6), 2);
    assert_eq!(map.visibility(7), 2);
    assert_eq!(map.visibility(11), 1);
    assert_eq!(map.visibility(20), 1);
    assert_eq!(map.visibility(24), 1);

    assert_eq!(map.visibility(17), 2);
}

#[test]
fn test_scenic_score() {
    let map = include_str!("testdata.txt").parse::<Map>().unwrap();

    // assert_eq!(map.scenic_score(0), 0);
    // assert_eq!(map.scenic_score(6), 4);
    assert_eq!(map.visibility_top(17), 2);
    assert_eq!(map.visibility_top(7), 1);

    assert_eq!(map.visibility_bottom(17), 1);
    assert_eq!(map.visibility_bottom(7), 2);

    assert_eq!(map.visibility_left(17), 1);
    assert_eq!(map.visibility_left(7), 1);

    assert_eq!(map.visibility_right(1), 2);
    assert_eq!(map.visibility_right(19), 0);
    assert_eq!(map.visibility_right(18), 1);
    assert_eq!(map.visibility_right(17), 2);
    assert_eq!(map.visibility_right(15), 2);
    assert_eq!(map.visibility_right(20), 2);
    assert_eq!(map.visibility_right(7), 2);

    // assert_eq!(map.scenic_score(6), 6);
}
