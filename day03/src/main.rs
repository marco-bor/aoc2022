fn main() {
    let input = include_str!("input.txt");

    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

const fn priority(char: char) -> u32 {
    match char {
        'a'..='z' => 1 + (char as u32) - ('a' as u32),
        'A'..='Z' => 27 + (char as u32) - ('A' as u32),
        _ => 0,
    }
}

fn problem1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .flat_map(|x| x.0.chars().find(|c| x.1.contains(*c)))
        .map(|c| priority(c))
        .sum()
}

fn problem2(input: &str) -> u32 {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks_exact(3)
        .flat_map(|g| find_badge(g))
        .map(|badge| priority(badge))
        .sum()
}

fn find_badge(group: &[&str]) -> Option<char> {
    if let [first, second, third] = group {
        first
            .chars()
            .find(|&c| second.contains(c) && third.contains(c))
    } else {
        None
    }
}

#[test]
fn test_priorities() {
    assert_eq!(priority('a'), 1);
    assert_eq!(priority('z'), 26);
    assert_eq!(priority('A'), 27);
    assert_eq!(priority('Z'), 52);
}
