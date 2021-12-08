use std::collections::HashSet;

use anyhow::Result;
use arrayvec::ArrayVec;

pub const INPUT_PATH: &str = "inputs/day/8/input";

pub fn part_one(contents: &str) -> Result<usize> {
    Ok(count_simple_digits(load_displays(contents)?.as_slice()))
}

pub fn part_two(contents: &str) -> Result<usize> {
    Ok(load_displays(contents)?
        .iter()
        .map(|(signals, digits)| displayed_value(signals, digits))
        .sum())
}

fn count_simple_digits(input: &[([&str; 10], [&str; 4])]) -> usize {
    input
        .iter()
        .map(|(_, digits)| digits)
        .flat_map(|digits| identify_simple_digits(digits).into_iter())
        .filter(|digit| {
            *digit == Some(1) || *digit == Some(4) || *digit == Some(7) || *digit == Some(8)
        })
        .map(|digit| if digit.is_some() { 1 } else { 0 })
        .sum()
}

fn displayed_value(signals: &[&str; 10], digits: &[&str; 4]) -> usize {
    let signals = deduce_digits(signals);
    digits
        .iter()
        .enumerate()
        .map(|(m, digit)| {
            let magnitude = 10usize.pow(3 - m as u32);
            let this_signal: HashSet<char> = digit.chars().collect();
            let digit = signals
                .iter()
                .enumerate()
                .find(|(_, signal)| **signal == this_signal)
                .map(|(i, _)| i)
                .expect("didn't find matching signal");
            magnitude * digit
        })
        .sum()
}

fn deduce_digits(signals: &[&str; 10]) -> [HashSet<char>; 10] {
    let signal_sets: ArrayVec<HashSet<char>, 10> = signals
        .iter()
        .map(|signal| signal.chars().collect())
        .collect();

    let one = signal_sets
        .iter()
        .find(|digit| digit.len() == 2)
        .expect("Missing digit")
        .clone();

    let four = signal_sets
        .iter()
        .find(|digit| digit.len() == 4)
        .expect("Missing digit")
        .clone();

    let seven = signal_sets
        .iter()
        .find(|digit| digit.len() == 3)
        .expect("missing digit")
        .clone();

    let eight = signal_sets
        .iter()
        .find(|digit| digit.len() == 7)
        .expect("missing digit")
        .clone();

    // of 5 length digits (2, 3, 5) only 3 will contain 1's signals
    let three = signal_sets
        .iter()
        .find(|digit| digit.len() == 5 && one.is_subset(digit))
        .expect("missing digit")
        .clone();

    let b_d: HashSet<char> = four.difference(&one).copied().collect(); // of 5 length digits (2, 3, 5) only 5 will contain these signals
    let five = signal_sets
        .iter()
        .find(|digit| digit.len() == 5 && b_d.is_subset(digit))
        .expect("missing digit")
        .clone();

    let two = signal_sets
        .iter()
        .find(|digit| digit.len() == 5 && !one.is_subset(digit) && !b_d.is_subset(digit))
        .expect("missing digit")
        .clone();

    let six = signal_sets
        .iter()
        .find(|digit| digit.len() == 6 && !one.is_subset(digit))
        .expect("missing digit")
        .clone();

    let c_e: HashSet<char> = eight.difference(&five).copied().collect();
    let zero = signal_sets
        .iter()
        .find(|digit| digit.len() == 6 && one.is_subset(digit) && c_e.is_subset(digit))
        .expect("missing digit")
        .clone();

    let nine = signal_sets
        .iter()
        .find(|digit| digit.len() == 6 && one.is_subset(digit) && !c_e.is_subset(digit))
        .expect("missing digit")
        .clone();

    [zero, one, two, three, four, five, six, seven, eight, nine]
}

fn identify_simple_digits(in_digits: &[&str; 4]) -> [Option<usize>; 4] {
    let mut digits = [None; 4];
    (0..in_digits.len()).for_each(|i| {
        digits[i] = match in_digits[i].len() {
            2 => Some(1),
            4 => Some(4),
            3 => Some(7),
            7 => Some(8),
            _ => None,
        };
    });
    digits
}

fn load_displays(contents: &str) -> Result<Vec<([&str; 10], [&str; 4])>> {
    contents
        .lines()
        .map(|line| {
            let (patterns, digits) = line
                .split_once(" | ")
                .ok_or_else(|| anyhow::anyhow!("Missing delimiter"))?;
            let patterns: ArrayVec<_, 10> = patterns.splitn(10, ' ').collect();
            let patterns = patterns
                .into_inner()
                .map_err(|_| anyhow::anyhow!("not enough signal patterns"))?; // Why can't I just use ? against into_inner?
            let digits: ArrayVec<_, 4> = digits.splitn(4, ' ').collect();
            let digits = digits
                .into_inner()
                .map_err(|_| anyhow::anyhow!("not enough signal patterns"))?; // Why can't I just use ? against into_inner?
            Ok((patterns, digits))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_load_displays() {
        let input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

        let expected = (
            [
                "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb",
                "ab",
            ],
            ["cdfeb", "fcadb", "cdfeb", "cdbaf"],
        );

        assert_eq!(vec![expected], load_displays(input).unwrap());
    }

    #[test]
    fn test_count_simple_digits() {
        let input = read_to_string("fixtures/digits.txt").unwrap();
        assert_eq!(
            26,
            count_simple_digits(load_displays(input.as_str()).unwrap().as_slice())
        );
    }

    #[test]
    fn test_displayed_value() {
        let signals = [
            "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab",
        ];

        let digits = ["cdfeb", "fcadb", "cdfeb", "cdbaf"];

        assert_eq!(5353, displayed_value(&signals, &digits));
    }

    #[test]
    fn test_deduce_digits() {
        let input = [
            "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab",
        ];

        let expected: [HashSet<char>; 10] = [
            "cagedb".chars().collect(),
            "ab".chars().collect(),
            "gcdfa".chars().collect(),
            "fbcad".chars().collect(),
            "eafb".chars().collect(),
            "cdfbe".chars().collect(),
            "cdfgeb".chars().collect(),
            "dab".chars().collect(),
            "acedgfb".chars().collect(),
            "cefabd".chars().collect(),
        ];
        assert_eq!(expected, deduce_digits(&input));
    }
}
