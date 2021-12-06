use std::{collections::HashMap, num::ParseIntError};

use anyhow::Result;

pub const INPUT_PATH: &str = "inputs/day/6/input";

pub fn part_one(contents: &str) -> Result<usize> {
    let fish: std::result::Result<Vec<usize>, ParseIntError> =
        contents.trim().split(',').map(|s| s.parse()).collect();
    Ok(simulate(fish?.as_slice(), 80))
}

pub fn part_two(contents: &str) -> Result<usize> {
    let fish: std::result::Result<Vec<usize>, ParseIntError> =
        contents.trim().split(',').map(|s| s.parse()).collect();
    Ok(simulate(fish?.as_slice(), 256))
}

fn simulate(fish: &[usize], days: usize) -> usize {
    let mut fish_generations: HashMap<usize, usize> =
        fish.iter().fold(HashMap::new(), |mut generations, fish| {
            *generations.entry(*fish).or_default() += 1;
            generations
        });
    (0..days).for_each(|_| progress_day(&mut fish_generations));
    fish_generations.values().sum()
}

fn progress_day(generations: &mut HashMap<usize, usize>) {
    let resetting_fish = generations.get(&0).copied().unwrap_or(0);
    (1..=8).for_each(|generation| {
        let fish = generations.get(&generation).copied().unwrap_or(0);
        generations.insert(generation - 1, fish);
    });
    *generations.entry(6).or_default() += resetting_fish;
    generations.insert(8, resetting_fish);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[usize] = &[3, 4, 3, 1, 2];

    #[test]
    fn test_simulate() {
        assert_eq!(5934, simulate(SAMPLE, 80));
    }

    #[test]
    fn test_progress_day() {
        let mut generations: HashMap<_, _> = (&[(0, 1), (1, 1), (2, 2), (3, 1)])
            .iter()
            .copied()
            .collect();
        let expected_generations: HashMap<_, _> = (&[
            (0, 1),
            (1, 2),
            (2, 1),
            (3, 0),
            (4, 0),
            (5, 0),
            (6, 1),
            (7, 0),
            (8, 1),
        ])
            .iter()
            .copied()
            .collect();

        progress_day(&mut generations);
        assert_eq!(expected_generations, generations);
    }
}
