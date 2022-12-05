fn main() {
    let input = include_str!("input.txt");

    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> u32 {
    0
}

fn problem2(input: &str) -> u32 {
    0
}

#[test]
fn test_problem1() {
    assert_eq!(problem1(include_str!("testdata.txt")), 0);
}

#[test]
fn test_problem2() {
    assert_eq!(problem2(include_str!("testdata.txt")), 0);
}