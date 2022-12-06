use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");

    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> u32 {
    let first_unique_pattern = find_unique_pattern(input, 4).expect("Cannot find pattern") as u32;
    first_unique_pattern + 4
}

fn problem2(input: &str) -> u32 {
    let first_unique_pattern = find_unique_pattern(input, 14).expect("Cannot find pattern") as u32;
    first_unique_pattern + 14
}

#[test]
fn test_problem1() {
    assert_eq!(problem1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(problem1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(problem1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(problem1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
}

#[test]
fn test_problem2() {
    assert_eq!(problem2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(problem2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    assert_eq!(problem2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    assert_eq!(problem2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    assert_eq!(problem2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
}

fn find_unique_pattern(s: &str, size: usize) -> Option<usize> {
    s.as_bytes()
        .windows(size)
        .enumerate()
        .filter(|(_, n)| n.iter().unique().count() == size)
        .map(|(i, _)| i)
        .next()
}
