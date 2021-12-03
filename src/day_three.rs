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
    let generator_rating = calculate_generator_rating(diagnostics.as_slice(), diagnostic_length)?;
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
    let rating: Vec<String> = (0..diagnostic_length as usize).fold(candidates, |candidates, i| {
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
    let rating: Vec<String> = (0..diagnostic_length as usize).fold(candidates, |candidates, i| {
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
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
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
