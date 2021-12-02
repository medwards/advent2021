use std::fs::read_to_string;

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
}

fn load_integers(path: &str) -> Result<Vec<usize>> {
    let contents = read_to_string(path)?;
    let integers: Result<Vec<_>> = contents
        .trim()
        .split("\n")
        .map(|s| s.parse().map_err(|e| anyhow::Error::new(e)))
        .collect();
    integers
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

    pub const INPUT_PATH: &'static str = "inputs/day/1/input";

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

    pub const INPUT_PATH: &'static str = "inputs/day/2/input";

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
            .split("\n")
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
