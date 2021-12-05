use std::cmp::{max, min};
use std::collections::HashMap;
use std::num::ParseIntError;

use anyhow::Result;

pub const INPUT_PATH: &str = "inputs/day/5/input";

pub fn part_one(contents: &str) -> Result<usize> {
    let mut input = load_endpoints(contents)?;
    input.retain(|coords| coords[0][0] == coords[1][0] || coords[0][1] == coords[1][1]);
    Ok(count_overlap(input.as_slice()))
}

pub fn part_two(contents: &str) -> Result<usize> {
    Ok(count_overlap(load_endpoints(contents)?.as_slice()))
}

fn count_overlap(endpoints: &[[[usize; 2]; 2]]) -> usize {
    endpoints
        .iter()
        .flat_map(|pair| {
            // let coords = vec![pair[0], pair[1]];
            let (x1, y1, x2, y2) = (pair[0][0], pair[0][1], pair[1][0], pair[1][1]);
            let coords: Vec<_> = if x1 == x2 {
                (min(y1, y2)..=max(y1, y2)).map(|y| [x1, y]).collect()
            } else if y1 == y2 {
                (min(x1, x2)..=max(x1, x2)).map(|x| [x, y1]).collect()
            } else {
                let x_delta: i32 = x2 as i32 - x1 as i32;
                let y_delta: i32 = y2 as i32 - y1 as i32;
                (0..=x_delta.abs())
                    .map(|i| {
                        let x = x1 as i32 + (i * x_delta.signum());
                        let y = y1 as i32 + (i * y_delta.signum());
                        [x as usize, y as usize]
                    })
                    .collect()
            };
            coords
        })
        .fold(HashMap::<[usize; 2], usize>::new(), |mut counts, coord| {
            *counts.entry(coord).or_default() += 1;
            counts
        })
        .iter()
        .map(|(_coord, count)| if *count > 1 { 1 } else { 0 })
        .sum()
}

fn load_endpoints(content: &str) -> std::result::Result<Vec<[[usize; 2]; 2]>, ParseIntError> {
    content
        .lines()
        .map(|s| {
            let mut row = [[0, 0], [0, 0]];
            // will panic if more numbers than expected, will result in a partial row if there are
            // less numbers than expected
            s.split(" -> ").enumerate().try_for_each(|(i, position)| {
                position
                    .split(',')
                    .enumerate()
                    .try_for_each(|(j, coordinate)| {
                        let coordinate = coordinate.parse()?;
                        row[i][j] = coordinate;
                        Ok(())
                    })?;
                Ok(())
            })?;
            Ok(row)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::read_to_string;

    const SAMPLE_INPUT: &[[[usize; 2]; 2]] = &[
        [[0, 9], [5, 9]],
        [[8, 0], [0, 8]],
        [[9, 4], [3, 4]],
        [[2, 2], [2, 1]],
        [[7, 0], [7, 4]],
        [[6, 4], [2, 0]],
        [[0, 9], [2, 9]],
        [[3, 4], [1, 4]],
        [[0, 0], [8, 8]],
        [[5, 5], [8, 2]],
    ];

    #[test]
    fn test_count_overlaps() {
        assert_eq!(12, count_overlap(SAMPLE_INPUT));
    }

    #[test]
    fn test_count_point_overlaps() {
        let input = &[[[0, 0], [0, 0]], [[0, 0], [0, 0]]];
        assert_eq!(1, count_overlap(input));
    }

    #[test]
    fn test_count_row_column_overlaps() {
        let mut input = SAMPLE_INPUT.to_vec();
        input.retain(|coords| coords[0][0] == coords[1][0] || coords[0][1] == coords[1][1]);
        assert_eq!(5, count_overlap(input.as_slice()));
    }

    #[test]
    fn test_load_endpoints() {
        assert_eq!(
            SAMPLE_INPUT,
            load_endpoints(read_to_string("fixtures/vents.txt").unwrap().as_str())
                .unwrap()
                .as_slice()
        );
    }
}
