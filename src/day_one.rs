use anyhow::Result;

use crate::load_integers;

pub const INPUT_PATH: &str = "inputs/day/1/input";

pub fn part_one(contents: &str) -> Result<usize> {
    let nums = load_integers(contents)?;
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
