use std::num::ParseIntError;

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
    let mut fish_generations: [usize; 9] = [0; 9];
    fish.iter().for_each(|fish| {
        fish_generations[*fish] += 1;
    });
    (0..days).for_each(|_| progress_day(&mut fish_generations));
    fish_generations.iter().sum()
}

fn progress_day(generations: &mut [usize; 9]) {
    let resetting_fish = generations[0];
    (1..=8).for_each(|generation| {
        let fish = generations[generation];
        generations[generation - 1] = fish;
    });
    generations[6] += resetting_fish;
    generations[8] = resetting_fish;
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
        let mut generations: [usize; 9] = [1, 1, 2, 1, 0, 0, 0, 0, 0];
        let expected_generations: [usize; 9] = [1, 2, 1, 0, 0, 0, 1, 0, 1];

        progress_day(&mut generations);
        assert_eq!(expected_generations, generations);
    }
}
