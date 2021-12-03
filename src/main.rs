use std::{fs::read_to_string, str::FromStr};

use anyhow::Result;

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

mod day_one {
    use super::load_integers;
    use anyhow::Result;

    pub const INPUT_PATH: &str = "inputs/day/1/input";

    pub fn part_one(path: &str) -> Result<usize> {
        let nums = load_integers(path)?;
        Ok(count_increases(nums.as_slice()))
    }

    pub fn part_two(path: &str) -> Result<usize> {
        let nums = load_integers(path)?;
        Ok(count_window_increases(nums.as_slice()))
    }

    fn count_increases(nums: &[usize]) -> usize {
        let pairs_iter = nums.iter().zip(nums.iter().skip(1));
        pairs_iter.fold(0, |count, (left, right)| {
            count + if left < right { 1 } else { 0 }
        })
    }

    fn count_window_increases(nums: &[usize]) -> usize {
        let window_sums: Vec<_> = nums.windows(3).map(|w| w.iter().sum()).collect();

        count_increases(window_sums.as_slice())
    }

    #[cfg(test)]
    mod tests {
        use super::count_increases;
        use super::count_window_increases;

        const EXAMPLE_INPUT: &'static [usize] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        #[test]
        fn test_example_part1() {
            assert_eq!(7, count_increases(EXAMPLE_INPUT));
        }

        #[test]
        fn test_example_part2() {
            assert_eq!(5, count_window_increases(EXAMPLE_INPUT));
        }
    }
}

mod day_two {
    use anyhow::{anyhow, Error, Result};
    use std::fs::read_to_string;

    pub const INPUT_PATH: &str = "inputs/day/2/input";

    pub fn part_one(path: &str) -> Result<usize> {
        let directions = read_to_directions(path)?;
        let (x, y) = calculate_position(directions.as_slice());
        Ok(x * y)
    }

    pub fn part_two(path: &str) -> Result<usize> {
        let directions = read_to_directions(path)?;
        let (x, y) = calculate_aimed_position(directions.as_slice());
        Ok(x * y)
    }

    // Assumes: depth cannot be less than 0
    #[derive(Debug, PartialEq)]
    enum Direction {
        Forward(usize),
        Down(usize),
        Up(usize),
    }

    impl TryFrom<&str> for Direction {
        type Error = Error;
        fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
            let (dir, delta) = value
                .split_once(" ")
                .ok_or_else(|| anyhow!("Malformed input"))?;
            let delta: usize = delta.parse()?;
            match dir {
                "forward" => Ok(Direction::Forward(delta)),
                "up" => Ok(Direction::Up(delta)),
                "down" => Ok(Direction::Down(delta)),
                other => Err(anyhow!("Malformed input, unexpected direction {}", other)),
            }
        }
    }

    fn calculate_position(directions: &[Direction]) -> (usize, usize) {
        use Direction::*;
        directions
            .iter()
            .fold((0, 0), |(x, y), direction| match direction {
                Forward(delta) => (x + delta, y),
                Down(delta) => (x, y + delta),
                Up(delta) => (x, y - delta),
            })
    }

    fn calculate_aimed_position(directions: &[Direction]) -> (usize, usize) {
        use Direction::*;
        let aimed_position =
            directions
                .iter()
                .fold((0, 0, 0), |(x, y, aim), direction| match direction {
                    Forward(delta) => (x + delta, y + (aim * delta), aim),
                    Down(delta) => (x, y, aim + delta),
                    Up(delta) => (x, y, aim - delta),
                });
        (aimed_position.0, aimed_position.1)
    }

    fn read_to_directions(path: &str) -> Result<Vec<Direction>> {
        let contents = read_to_string(path)?;
        let directions: Result<Vec<_>> = contents
            .trim()
            .split('\n')
            .map(Direction::try_from)
            .collect();
        directions
    }

    #[cfg(test)]
    mod tests {
        use super::{calculate_aimed_position, calculate_position, read_to_directions, Direction};

        const EXAMPLE_INPUT: &'static [Direction] = &[
            Direction::Forward(5),
            Direction::Down(5),
            Direction::Forward(8),
            Direction::Up(3),
            Direction::Down(8),
            Direction::Forward(2),
        ];

        #[test]
        fn test_calculate_position() {
            assert_eq!((15, 10), calculate_position(EXAMPLE_INPUT));
        }

        #[test]
        fn test_calculate_aimed_position() {
            assert_eq!((15, 60), calculate_aimed_position(EXAMPLE_INPUT));
        }

        #[test]
        fn test_read_to_directions() {
            assert_eq!(
                EXAMPLE_INPUT,
                read_to_directions("fixtures/submarine_directions.txt")
                    .unwrap()
                    .as_slice()
            );
        }
    }
}

mod day_three {
    use anyhow::Result;
    use std::{num::ParseIntError, ops::BitXor};

    use crate::read_to_lines;

    pub const INPUT_PATH: &str = "inputs/day/3/input";

    pub fn part_one(path: &str) -> Result<u32> {
        let diagnostics = read_to_lines(path)?;
        let diagnostic_length: u32 = diagnostics.get(0).map(|d| d.len() as u32).unwrap_or(0);
        let gamma_rate = calculate_gamma_rate(diagnostics.as_slice(), diagnostic_length)?;
        let epsilon_rate = epsilon_rate_from_gamma_rate(gamma_rate, diagnostic_length)?;
        Ok(gamma_rate * epsilon_rate)
    }

    pub fn part_two(path: &str) -> Result<u32> {
        let diagnostics = read_to_lines(path)?;
        let diagnostic_length: u32 = diagnostics.get(0).map(|d| d.len() as u32).unwrap_or(0);
        let generator_rating =
            calculate_generator_rating(diagnostics.as_slice(), diagnostic_length)?;
        let scrubber_rating = calculate_scrubber_rating(diagnostics.as_slice(), diagnostic_length)?;
        Ok(generator_rating * scrubber_rating)
    }

    fn calculate_gamma_rate<T: AsRef<str>>(
        diags: &[T],
        diagnostic_length: u32,
    ) -> Result<u32, ParseIntError> {
        let rate: String = (0..diagnostic_length as usize)
            .map(|i| {
                if is_one_most_common(i, diags, true) {
                    "1"
                } else {
                    "0"
                }
            })
            .collect();
        u32::from_str_radix(rate.as_str(), 2)
    }

    fn is_one_most_common<T: AsRef<str>>(index: usize, diagnostics: &[T], bias: bool) -> bool {
        let ones_count = diagnostics
            .iter()
            .flat_map(|d| {
                d.as_ref()
                    .chars()
                    .nth(index)
                    .and_then(|c| if c == '1' { Some('1') } else { None })
            })
            .count();
        if diagnostics.len() % 2 == 0 && ones_count == diagnostics.len() / 2 {
            bias
        } else {
            ones_count > diagnostics.len() / 2
        }
    }

    fn epsilon_rate_from_gamma_rate(gamma_rate: u32, diagnostic_length: u32) -> Result<u32> {
        // swap the 'most common' bytes but not the bytes outside of diagnostic length
        // -- epsilon rate is just whatever is left from gamma
        let epsilon_mask: String = (0..u32::BITS)
            .map(|i| {
                if i < u32::BITS - diagnostic_length {
                    "0"
                } else {
                    "1"
                }
            })
            .collect();
        let epsilon_mask = u32::from_str_radix(epsilon_mask.as_str(), 2)?;
        Ok(gamma_rate.bitxor(epsilon_mask))
    }

    fn calculate_generator_rating<T: AsRef<str>>(
        diagnostics: &[T],
        diagnostic_length: u32,
    ) -> Result<u32, ParseIntError> {
        let candidates = diagnostics.iter().map(|s| s.as_ref().to_string()).collect();
        let rating: Vec<String> =
            (0..diagnostic_length as usize).fold(candidates, |candidates, i| {
                if candidates.len() == 1 {
                    return candidates;
                }
                let ones_criteria = is_one_most_common(i, candidates.as_slice(), true);
                filter_candidates(i, ones_criteria, candidates.as_slice())
            });
        u32::from_str_radix(rating.get(0).unwrap(), 2)
    }

    fn calculate_scrubber_rating<T: AsRef<str>>(
        diagnostics: &[T],
        diagnostic_length: u32,
    ) -> Result<u32, ParseIntError> {
        let candidates = diagnostics.iter().map(|s| s.as_ref().to_string()).collect();
        let rating: Vec<String> =
            (0..diagnostic_length as usize).fold(candidates, |candidates, i| {
                if candidates.len() == 1 {
                    return candidates;
                }
                let ones_criteria = !is_one_most_common(i, candidates.as_slice(), true);
                filter_candidates(i, ones_criteria, candidates.as_slice())
            });
        u32::from_str_radix(rating.get(0).unwrap(), 2)
    }

    fn filter_candidates<T: AsRef<str>>(
        index: usize,
        ones_criteria: bool,
        diagnostics: &[T],
    ) -> Vec<String> {
        diagnostics
            .iter()
            .filter(|d| {
                let criteria_char = d
                    .as_ref()
                    .chars()
                    .nth(index)
                    .unwrap_or_else(|| panic!("index {} in diagnostic didn't exist", index));
                (ones_criteria && criteria_char == '1') || (!ones_criteria && criteria_char == '0')
            })
            .map(|s| s.as_ref().to_string())
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const EXAMPLE_INPUT: &'static [&str] = &[
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        #[test]
        fn test_is_one_most_common() {
            assert_eq!(true, is_one_most_common(0, EXAMPLE_INPUT, true));
        }

        #[test]
        fn test_calculate_gamma_rate() {
            assert_eq!(22, calculate_gamma_rate(EXAMPLE_INPUT, 5).unwrap());
        }

        #[test]
        fn test_epsilon_rate_from_gamma_rate() {
            assert_eq!(9, epsilon_rate_from_gamma_rate(22, 5).unwrap());
        }

        #[test]
        fn test_calculate_generator_rating() {
            assert_eq!(23, calculate_generator_rating(EXAMPLE_INPUT, 5).unwrap());
        }

        #[test]
        fn test_calculate_scrubber_rating() {
            assert_eq!(10, calculate_scrubber_rating(EXAMPLE_INPUT, 5).unwrap());
        }
    }
}
