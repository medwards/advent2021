use std::cmp::max;

use anyhow::Result;

pub const INPUT_PATH: &str = "inputs/day/14/input";

pub fn part_one(contents: &str) -> Result<usize> {
    let (template, rules) = load_polymerization(contents)?;

    let counts = process_polymer(template.as_slice(), rules.as_slice(), 10);

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

pub fn part_two(contents: &str) -> Result<usize> {
    let (template, rules) = load_polymerization(contents)?;

    let counts = process_polymer(template.as_slice(), rules.as_slice(), 40);

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

fn process_polymer(template: &[u8], rules: &[(u8, u8, u8)], iterations: usize) -> Vec<usize> {
    let pairs = template_to_pairs(template, rules);

    let pairs = (0..iterations).fold(pairs, |pairs, _| apply_rules(&pairs, rules));

    count_from_pairs(&pairs, template)
}

fn apply_rules(pairs: &Vec<Vec<usize>>, rules: &[(u8, u8, u8)]) -> Vec<Vec<usize>> {
    let mut result = vec![vec![0; pairs.len()]; pairs.len()];
    pairs.iter().enumerate().for_each(|(i, row)| {
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

fn template_to_pairs(template: &[u8], rules: &[(u8, u8, u8)]) -> Vec<Vec<usize>> {
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
    fn test_template_to_pairs() {
        let (template, rules) =
            load_polymerization(read_to_string("fixtures/polymer.txt").unwrap().as_str()).unwrap();

        let mut expected = vec![vec![0; 14]; 14];
        *expected.get_mut(13).unwrap().get_mut(13).unwrap() = 1;
        *expected.get_mut(13).unwrap().get_mut(2).unwrap() = 1;
        *expected.get_mut(2).unwrap().get_mut(1).unwrap() = 1;

        assert_eq!(
            expected,
            template_to_pairs(template.as_slice(), rules.as_slice())
        );
    }

    #[test]
    fn test_apply_rules() {
        let (template, rules) =
            load_polymerization(read_to_string("fixtures/polymer.txt").unwrap().as_str()).unwrap();

        let expected = template_to_pairs("NCNBCHB".as_bytes(), rules.as_slice());
        let mut old_counts = template_to_pairs(template.as_slice(), rules.as_slice());
        let counts = apply_rules(&old_counts, rules.as_slice());
        assert_eq!(expected, counts);

        let expected = template_to_pairs("NBCCNBBBCBHCB".as_bytes(), rules.as_slice());
        old_counts = counts;
        let counts = apply_rules(&old_counts, rules.as_slice());
        assert_eq!(expected, counts);

        let expected = template_to_pairs("NBBBCNCCNBBNBNBBCHBHHBCHB".as_bytes(), rules.as_slice());
        old_counts = counts;
        let counts = apply_rules(&old_counts, rules.as_slice());
        assert_eq!(expected, counts);

        let expected = template_to_pairs(
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".as_bytes(),
            rules.as_slice(),
        );
        old_counts = counts;
        let counts = apply_rules(&old_counts, rules.as_slice());
        assert_eq!(expected, counts);
    }

    #[test]
    fn test_counts_from_pairs() {
        let (_, rules) =
            load_polymerization(read_to_string("fixtures/polymer.txt").unwrap().as_str()).unwrap();

        let pairs = template_to_pairs(
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
