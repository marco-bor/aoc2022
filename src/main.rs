fn main() {
    let input = include_str!("input.txt");

    problem1(input);
    problem2(input);
}

fn problem1(input: &str) {
    let mut current_calories = 0u32;
    let mut max_calories = 0u32;

    input.lines().for_each(|s| {
        if s.is_empty() {
            max_calories = max_calories.max(current_calories);
            current_calories = 0;
        } else if let Ok(parsed) = s.parse::<u32>() {
            current_calories += parsed;
        } else {
            println!("Could not parse line '{}', skipping", s)
        }
    });

    // fix: file not ending with newline
    if current_calories > 0 {
        max_calories = max_calories.max(current_calories);
    }

    println!("Answer 1 is {}", max_calories)
}

fn problem2(input: &str) {
    let mut current_calories = 0u32;
    let mut calories: Vec<u32> = vec![];

    input.lines().for_each(|s| {
        if s.is_empty() {
            calories.push(current_calories);
            current_calories = 0;
        } else if let Ok(parsed) = s.parse::<u32>() {
            current_calories += parsed;
        } else {
            println!("Could not parse line '{}', skipping", s)
        }
    });

    // fix: file not ending with newline
    if current_calories > 0 {
        calories.push(current_calories);
    }

    calories.sort();
    let top3: u32 = calories.iter().rev().take(3).sum();

    println!("Answer 2 is {}", top3)
}
