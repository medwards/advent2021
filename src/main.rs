use std::{fs::read_to_string, str::FromStr};

use anyhow::Result;

mod day_one;
mod day_three;
mod day_two;

fn main() {
    println!(
        "Day 1, Part One: {}",
        day_one::part_one(day_one::INPUT_PATH).unwrap()
    );
    println!(
        "Day 1, Part Two: {}",
        day_one::part_two(day_one::INPUT_PATH).unwrap()
    );
    println!(
        "Day 2, Part One: {}",
        day_two::part_one(day_two::INPUT_PATH).unwrap()
    );
    println!(
        "Day 2, Part Two: {}",
        day_two::part_two(day_two::INPUT_PATH).unwrap()
    );
    println!(
        "Day 3, Part One: {}",
        day_three::part_one(day_three::INPUT_PATH).unwrap()
    );
    println!(
        "Day 3, Part Two: {}",
        day_three::part_two(day_three::INPUT_PATH).unwrap()
    );
}

fn load_integers(path: &str) -> Result<Vec<usize>> {
    let contents = read_to_string(path)?;
    let integers: Result<Vec<_>> = contents
        .trim()
        .split('\n')
        .map(|s| s.parse().map_err(anyhow::Error::new))
        .collect();
    integers
}

fn read_to_lines(path: &str) -> Result<Vec<String>> {
    let contents = read_to_string(path)?;
    let lines: Vec<_> = contents
        .trim()
        .split('\n')
        .flat_map(String::from_str) // can't fail
        .collect();
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::load_integers;

    #[test]
    fn test_load_integers() {
        let expected_output = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(
            expected_output,
            load_integers("fixtures/positive_integers.txt").expect("Unexpected failure")
        );
    }
}
