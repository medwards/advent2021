use std::cmp::max;
use std::collections::HashMap;
use std::iter::once;

use anyhow::Result;

pub const INPUT_PATH: &str = "inputs/day/14/input";

pub fn part_one(contents: &str) -> Result<usize> {
    let (template, rules) = load_polymerization(contents)?;
    let polymer = (0..10).fold(template, |template, _| {
        apply_rules(template.as_slice(), rules.as_slice())
    });

    let counts = polymer
        .iter()
        .fold(HashMap::<_, usize>::new(), |mut counts, element| {
            *counts.entry(element).or_default() += 1;
            counts
        });

    Ok(counts
        .values()
        .max()
        .ok_or_else(|| anyhow::anyhow!("empty polymer"))?
        - counts
            .values()
            .min()
            .ok_or_else(|| anyhow::anyhow!("empty_polymer"))?)
}

pub fn part_two(contents: &str) -> Result<usize> {
    let (template, rules) = load_polymerization(contents)?;

    let pairs = template_to_pair_counts(template.as_slice(), rules.as_slice());

    let pairs = (0..40).fold(pairs, |pairs, _| {
        apply_rules_count(&pairs, rules.as_slice())
    });

    let counts = count_from_pairs(&pairs, template.as_slice());

    Ok(counts
        .iter()
        .max()
        .ok_or_else(|| anyhow::anyhow!("empty polymer"))?
        - counts
            .iter()
            .filter(|count| **count != 0)
            .min()
            .ok_or_else(|| anyhow::anyhow!("empty_polymer"))?)
}

fn apply_rules(template: &[u8], rules: &[(u8, u8, u8)]) -> Vec<u8> {
    template
        .windows(2)
        .flat_map(|window| {
            let rule = rules
                .iter()
                .find(|rule| window[0] == rule.0 && window[1] == rule.1)
                .expect("No matching rule");
            vec![rule.0, rule.2]
        })
        .chain(once(*template.last().expect("empty template")))
        .collect()
}

fn apply_rules_count(pair_counts: &Vec<Vec<usize>>, rules: &[(u8, u8, u8)]) -> Vec<Vec<usize>> {
    let mut result = vec![vec![0; pair_counts.len()]; pair_counts.len()];
    pair_counts.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, count)| {
            if let Some((_, _, output)) = rules
                .iter()
                .find(|rule| rule.0 == i as u8 + b'A' && rule.1 == j as u8 + b'A')
            {
                let output_index = (output - b'A') as usize;

                *result.get_mut(i).unwrap().get_mut(output_index).unwrap() += *count;
                *result.get_mut(output_index).unwrap().get_mut(j).unwrap() += *count;
            } else {
                *result.get_mut(i).unwrap().get_mut(j).unwrap() += *count;
            }
        })
    });
    result
}

fn count_from_pairs(pairs: &Vec<Vec<usize>>, template: &[u8]) -> Vec<usize> {
    let mut counts = vec![0; pairs.len()];
    template
        .last()
        .iter()
        .for_each(|j| counts[(**j - b'A') as usize] += 1);

    pairs
        .iter()
        .enumerate()
        .for_each(|(i, row)| counts[i] += row.iter().sum::<usize>());
    counts
}

fn template_to_pair_counts(template: &[u8], rules: &[(u8, u8, u8)]) -> Vec<Vec<usize>> {
    let max = (*max(template.iter().max(), rules.iter().map(|(_, _, o)| o).max()).unwrap_or(&0)
        - b'A') as usize;
    let mut counts = vec![vec![0; max + 1]; max + 1];

    template.windows(2).for_each(|pair| {
        let i = (pair[0] - b'A') as usize;
        let j = (pair[1] - b'A') as usize;
        *counts.get_mut(i).unwrap().get_mut(j).unwrap() += 1;
    });

    counts
}

fn load_polymerization(contents: &str) -> Result<(Vec<u8>, Vec<(u8, u8, u8)>)> {
    let mut iter = contents.lines();

    let template = iter
        .nth(0)
        .ok_or_else(|| anyhow::anyhow!("empty input"))?
        .as_bytes()
        .to_vec();

    let rules: Result<Vec<_>> = iter
        .skip(1)
        .map(|line| {
            let parts = line
                .split_once(" -> ")
                .ok_or_else(|| anyhow::anyhow!("Missing rule delimiter"))?;
            let output = parts
                .1
                .as_bytes()
                .get(0)
                .ok_or_else(|| anyhow::anyhow!("Missing rule output"))?;
            let left = parts
                .0
                .as_bytes()
                .get(0)
                .ok_or_else(|| anyhow::anyhow!("Missing rule left input"))?;
            let right = parts
                .0
                .as_bytes()
                .get(1)
                .ok_or_else(|| anyhow::anyhow!("Missing rule right input"))?;
            Ok((*left, *right, *output))
        })
        .collect();

    Ok((template, rules?))
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_apply_rules() {
        let (template, rules) =
            load_polymerization(read_to_string("fixtures/polymer.txt").unwrap().as_str()).unwrap();

        assert_eq!(
            "NCNBCHB".as_bytes(),
            apply_rules(template.as_slice(), rules.as_slice())
        );
    }

    #[test]
    fn test_template_to_counts() {
        let (template, rules) =
            load_polymerization(read_to_string("fixtures/polymer.txt").unwrap().as_str()).unwrap();

        let mut expected = vec![vec![0; 14]; 14];
        *expected.get_mut(13).unwrap().get_mut(13).unwrap() = 1;
        *expected.get_mut(13).unwrap().get_mut(2).unwrap() = 1;
        *expected.get_mut(2).unwrap().get_mut(1).unwrap() = 1;

        assert_eq!(
            expected,
            template_to_pair_counts(template.as_slice(), rules.as_slice())
        );
    }

    #[test]
    fn test_apply_rules_count() {
        let (template, rules) =
            load_polymerization(read_to_string("fixtures/polymer.txt").unwrap().as_str()).unwrap();

        let expected = template_to_pair_counts("NCNBCHB".as_bytes(), rules.as_slice());
        let mut old_counts = template_to_pair_counts(template.as_slice(), rules.as_slice());
        let counts = apply_rules_count(&old_counts, rules.as_slice());
        assert_eq!(expected, counts);

        let expected = template_to_pair_counts("NBCCNBBBCBHCB".as_bytes(), rules.as_slice());
        old_counts = counts;
        let counts = apply_rules_count(&old_counts, rules.as_slice());
        assert_eq!(expected, counts);

        let expected =
            template_to_pair_counts("NBBBCNCCNBBNBNBBCHBHHBCHB".as_bytes(), rules.as_slice());
        old_counts = counts;
        let counts = apply_rules_count(&old_counts, rules.as_slice());
        assert_eq!(expected, counts);

        let expected = template_to_pair_counts(
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".as_bytes(),
            rules.as_slice(),
        );
        old_counts = counts;
        let counts = apply_rules_count(&old_counts, rules.as_slice());
        assert_eq!(expected, counts);
    }

    #[test]
    fn test_pair_counts_to_counts() {
        let (_, rules) =
            load_polymerization(read_to_string("fixtures/polymer.txt").unwrap().as_str()).unwrap();

        let pairs = template_to_pair_counts(
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".as_bytes(),
            rules.as_slice(),
        );

        let mut expected = vec![0; pairs.len()];
        "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
            .as_bytes()
            .iter()
            .for_each(|element| expected.as_mut_slice()[(element - b'A') as usize] += 1);

        assert_eq!(
            expected,
            count_from_pairs(
                &pairs,
                "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".as_bytes()
            )
        );
    }
}
