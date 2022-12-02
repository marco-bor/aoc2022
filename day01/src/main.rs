fn main() {
    let input = include_str!("input.txt");

    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|group| group.lines().flat_map(|s| s.parse::<u32>()).sum())
        .max()
        .unwrap()
}

fn problem2(input: &str) -> u32 {
    let mut calories: Vec<u32> = input
        .split("\n\n")
        .map(|group| group.lines().flat_map(|s| s.parse::<u32>()).sum())
        .collect();

    calories.sort_unstable();
    calories.iter().rev().take(3).sum()
}
