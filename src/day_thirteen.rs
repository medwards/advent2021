use std::ops::ControlFlow;

use anyhow::{Error, Result};

pub const INPUT_PATH: &str = "inputs/day/13/input";

pub fn part_one(contents: &str) -> Result<usize> {
    let (coords, folds) = load_paper(contents)?;
    Ok(folds
        .iter()
        .take(1)
        .fold(coords, |coords, fold| fold_paper(coords.as_slice(), fold))
        .len())
}

pub fn part_two(contents: &str) -> Result<usize> {
    let (coords, folds) = load_paper(contents)?;
    let result = folds
        .iter()
        .fold(coords, |coords, fold| fold_paper(coords.as_slice(), fold));
    // Disabled for benchmarks
    // println!("{:?}", result);
    Ok(result.len())
}

#[derive(Debug, PartialEq)]
pub enum Fold {
    Right(usize),
    Up(usize),
}

fn fold_paper(coords: &[(usize, usize)], fold: &Fold) -> Vec<(usize, usize)> {
    let mut folded: Vec<_> = coords
        .iter()
        .map(|(x, y)| match fold {
            Fold::Right(offset) => {
                if x > offset {
                    (offset - (x - offset), *y)
                } else {
                    (*x, *y)
                }
            }
            Fold::Up(offset) => {
                if y > offset {
                    (*x, offset - (y - offset))
                } else {
                    (*x, *y)
                }
            }
        })
        .collect();
    folded.sort_unstable();
    folded.dedup();
    folded
}

fn load_paper(contents: &str) -> Result<(Vec<(usize, usize)>, Vec<Fold>)> {
    let mut paper_iter = contents.lines();
    let fold_res = paper_iter.try_fold(Vec::new(), |mut coords, line| match line.split_once(',') {
        Some((x, y)) => {
            let x: usize = match x.parse() {
                Ok(x) => x,
                Err(e) => return ControlFlow::Break(Err(e)),
            };
            let y: usize = match y.parse() {
                Ok(y) => y,
                Err(e) => return ControlFlow::Break(Err(e)),
            };
            coords.push((x, y));
            ControlFlow::Continue(coords)
        }
        None => ControlFlow::Break(Ok(coords)),
    });

    let coords = match fold_res {
        ControlFlow::Continue(_) => panic!("unfinished coords iter"), // shouldn't happen
        ControlFlow::Break(res) => res?,
    };

    let folds: Result<Vec<_>> = paper_iter
        .map(|line| {
            let parts = line.split_once('=');
            // TODO: this awkwardness because `?` wasn't allowed inside the `map`. Why?
            let parts = if let Some(parts) = parts {
                parts
            } else {
                return Result::Err(anyhow::anyhow!("bad input"));
            };

            let fold_point: std::result::Result<usize, _> = parts.1.parse();
            let fold_point = if let Ok(fold_point) = fold_point {
                fold_point
            } else {
                return Result::Err(Error::new(fold_point.unwrap_err()));
            };

            Result::Ok(if parts.0.ends_with('x') {
                Fold::Right(fold_point)
            } else if parts.0.ends_with('y') {
                Fold::Up(fold_point)
            } else {
                return Result::Err(anyhow::anyhow!("bad input"));
            })
        })
        .collect();
    let folds = folds?;

    Ok((coords, folds))
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_load_paper() {
        let coords = vec![
            (6, 10),
            (0, 14),
            (9, 10),
            (0, 3),
            (10, 4),
            (4, 11),
            (6, 0),
            (6, 12),
            (4, 1),
            (0, 13),
            (10, 12),
            (3, 4),
            (3, 0),
            (8, 4),
            (1, 10),
            (2, 14),
            (8, 10),
            (9, 0),
        ];
        let folds = vec![Fold::Up(7), Fold::Right(5)];

        let expected = (coords, folds);

        let contents = read_to_string("fixtures/manual_page.txt").unwrap();
        assert_eq!(expected, load_paper(contents.as_str()).unwrap());
    }

    #[test]
    fn test_fold_paper() {
        let input = vec![
            (6, 10),
            (0, 14),
            (9, 10),
            (0, 3),
            (10, 4),
            (4, 11),
            (6, 0),
            (6, 12),
            (4, 1),
            (0, 13),
            (10, 12),
            (3, 4),
            (3, 0),
            (8, 4),
            (1, 10),
            (2, 14),
            (8, 10),
            (9, 0),
        ];

        assert_eq!(17, fold_paper(input.as_slice(), &Fold::Up(7)).len());
    }
}
