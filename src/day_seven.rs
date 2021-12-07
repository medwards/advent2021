use std::cmp::{max, min};
use std::num::ParseIntError;

use anyhow::Result;

pub const INPUT_PATH: &str = "inputs/day/7/input";

pub fn part_one(contents: &str) -> Result<usize> {
    let crabs: std::result::Result<Vec<usize>, ParseIntError> =
        contents.trim().split(',').map(|s| s.parse()).collect();
    Ok(least_fuel(crabs?.as_slice()))
}

pub fn part_two(contents: &str) -> Result<usize> {
    let crabs: std::result::Result<Vec<usize>, ParseIntError> =
        contents.trim().split(',').map(|s| s.parse()).collect();
    Ok(least_real_fuel(crabs?.as_slice()))
}

fn least_fuel(crabs: &[usize]) -> usize {
    let crab_max = crabs.iter().max().expect("No crabs!");
    (0..=*crab_max)
        .map(|pos| {
            crabs
                .iter()
                .map(|crab| max(*crab, pos) - min(*crab, pos))
                .sum()
        })
        .fold(0, |min_fuel, fuel_sum| {
            if fuel_sum < min_fuel || min_fuel == 0 {
                fuel_sum
            } else {
                min_fuel
            }
        })
}

fn least_real_fuel(crabs: &[usize]) -> usize {
    let crab_max = crabs.iter().max().expect("No crabs!");
    (0..=*crab_max)
        .map(|pos| {
            crabs
                .iter()
                .map(|crab| (1..=(max(*crab, pos) - min(*crab, pos))).sum::<usize>())
                .sum()
        })
        .fold(0, |min_fuel, fuel_sum| {
            if fuel_sum < min_fuel || min_fuel == 0 {
                fuel_sum
            } else {
                min_fuel
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_least_fuel() {
        let crabs = &[16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(37, least_fuel(crabs));
    }

    #[test]
    fn test_least_real_fuel() {
        let crabs = &[16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(168, least_real_fuel(crabs));
    }
}
