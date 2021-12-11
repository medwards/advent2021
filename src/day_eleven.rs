use anyhow::Result;

pub const INPUT_PATH: &str = "inputs/day/11/input";

pub fn part_one(contents: &str) -> Result<usize> {
    let grid = load_grid(contents);
    Ok((0..100)
        .fold((0, grid), |(flashes, grid), _| {
            let (new_flashes, new_grid) = increment(&grid);
            (flashes + new_flashes, new_grid)
        })
        .0)
}

pub fn part_two(contents: &str) -> Result<usize> {
    let mut grid = load_grid(contents);
    let mut iter = 0;
    loop {
        let (flashes, new_grid) = increment(&grid);
        iter += 1;
        if flashes == 100 {
            return Ok(iter);
        }
        grid = new_grid;
    }
}

fn load_grid(contents: &str) -> [[usize; 10]; 10] {
    let mut grid = [[0; 10]; 10];
    contents.lines().enumerate().for_each(|(i, row)| {
        row.chars().enumerate().for_each(|(j, character)| {
            grid[i][j] = match character {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                _ => panic!("malformed input"),
            }
        })
    });

    grid
}

// assumes no octopus has value 9
fn increment(octopus_grid: &[[usize; 10]; 10]) -> (usize, [[usize; 10]; 10]) {
    let mut inverted_octopus_grid = *octopus_grid;

    inverted_octopus_grid
        .as_mut_slice()
        .iter_mut()
        .for_each(|row| {
            row.as_mut_slice()
                .iter_mut()
                .for_each(|octopus| *octopus = 10 - *octopus)
        });

    inverted_octopus_grid
        .as_mut_slice()
        .iter_mut()
        .for_each(|row| {
            row.as_mut_slice()
                .iter_mut()
                .for_each(|octopus| *octopus = (*octopus).saturating_sub(1))
        });

    let mut flashed: Vec<_> = inverted_octopus_grid
        .as_slice()
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.as_slice()
                .iter()
                .enumerate()
                .flat_map(
                    |(j, octopus)| {
                        if *octopus == 0 {
                            Some((i, j))
                        } else {
                            None
                        }
                    },
                )
                .collect::<Vec<_>>() // TODO: why the collect
        })
        .collect();

    if !flashed.is_empty() {
        let mut flashed_count = flashed.len();
        loop {
            flashed = flashed
                .iter()
                .flat_map(|(i, j)| decrement_adjacent(&mut inverted_octopus_grid, *i, *j))
                .collect();

            if flashed.is_empty() {
                inverted_octopus_grid
                    .as_mut_slice()
                    .iter_mut()
                    .for_each(|row| {
                        row.as_mut_slice().iter_mut().for_each(|octopus| {
                            if *octopus == 0 {
                                *octopus = 0
                            } else {
                                *octopus = 10 - *octopus
                            }
                        })
                    });

                return (flashed_count, inverted_octopus_grid);
            } else {
                flashed_count += flashed.len();
            }
        }
    } else {
        inverted_octopus_grid
            .as_mut_slice()
            .iter_mut()
            .for_each(|row| {
                row.as_mut_slice().iter_mut().for_each(|octopus| {
                    if *octopus == 0 {
                        *octopus = 0
                    } else {
                        *octopus = 10 - *octopus
                    }
                })
            });
        (flashed.len(), inverted_octopus_grid)
    }
}

fn decrement_adjacent(
    octopus_grid: &mut [[usize; 10]; 10],
    i: usize,
    j: usize,
) -> Vec<(usize, usize)> {
    let mut flashed = vec![];
    if i > 0 {
        flashed.extend(decrement_octopus(octopus_grid, i - 1, j).iter());
    }
    if j > 0 {
        flashed.extend(decrement_octopus(octopus_grid, i, j - 1).iter());
    }
    if i > 0 && j > 0 {
        flashed.extend(decrement_octopus(octopus_grid, i - 1, j - 1).iter());
    }

    if i < 9 {
        flashed.extend(decrement_octopus(octopus_grid, i + 1, j).iter());
    }
    if j < 9 {
        flashed.extend(decrement_octopus(octopus_grid, i, j + 1).iter());
    }
    if i < 9 && j < 9 {
        flashed.extend(decrement_octopus(octopus_grid, i + 1, j + 1).iter());
    }

    if i < 9 && j > 0 {
        flashed.extend(decrement_octopus(octopus_grid, i + 1, j - 1).iter());
    }

    if i > 0 && j < 9 {
        flashed.extend(decrement_octopus(octopus_grid, i - 1, j + 1).iter());
    }
    flashed
}

fn decrement_octopus(
    octopus_grid: &mut [[usize; 10]; 10],
    i: usize,
    j: usize,
) -> Option<(usize, usize)> {
    let mut flashed = None;
    if octopus_grid[i][j] == 1 {
        flashed = Some((i, j)); // only return flashed if *this* decrement would cause it to go to 0
    }
    octopus_grid[i][j] = octopus_grid[i][j].saturating_sub(1);
    flashed
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_increment() {
        let mut input = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 1, 1, 1, 1, 0, 0, 0, 0],
            [0, 1, 9, 9, 9, 1, 0, 0, 0, 0],
            [0, 1, 9, 1, 9, 1, 0, 0, 0, 0],
            [0, 1, 9, 9, 9, 1, 0, 0, 0, 0],
            [0, 1, 1, 1, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        let mut expected = [
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 3, 4, 5, 4, 3, 1, 1, 1, 1],
            [1, 4, 0, 0, 0, 4, 1, 1, 1, 1],
            [1, 5, 0, 0, 0, 5, 1, 1, 1, 1],
            [1, 4, 0, 0, 0, 4, 1, 1, 1, 1],
            [1, 3, 4, 5, 4, 3, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];

        assert_eq!((9, expected), increment(&mut input));
    }

    #[test]
    fn test_increment() {
        let mut input = [
            [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ];

        let expected1 = [
            [6, 5, 9, 4, 2, 5, 4, 3, 3, 4],
            [3, 8, 5, 6, 9, 6, 5, 8, 2, 2],
            [6, 3, 7, 5, 6, 6, 7, 2, 8, 4],
            [7, 2, 5, 2, 4, 4, 7, 2, 5, 7],
            [7, 4, 6, 8, 4, 9, 6, 5, 8, 9],
            [5, 2, 7, 8, 6, 3, 5, 7, 5, 6],
            [3, 2, 8, 7, 9, 5, 2, 8, 3, 2],
            [7, 9, 9, 3, 9, 9, 2, 2, 4, 5],
            [5, 9, 5, 7, 9, 5, 9, 6, 6, 5],
            [6, 3, 9, 4, 8, 6, 2, 6, 3, 7],
        ];

        let expected2 = [
            [8, 8, 0, 7, 4, 7, 6, 5, 5, 5],
            [5, 0, 8, 9, 0, 8, 7, 0, 5, 4],
            [8, 5, 9, 7, 8, 8, 9, 6, 0, 8],
            [8, 4, 8, 5, 7, 6, 9, 6, 0, 0],
            [8, 7, 0, 0, 9, 0, 8, 8, 0, 0],
            [6, 6, 0, 0, 0, 8, 8, 9, 8, 9],
            [6, 8, 0, 0, 0, 0, 5, 9, 4, 3],
            [0, 0, 0, 0, 0, 0, 7, 4, 5, 6],
            [9, 0, 0, 0, 0, 0, 0, 8, 7, 6],
            [8, 7, 0, 0, 0, 0, 6, 8, 4, 8],
        ];

        let (count, mut step_two) = increment(&mut input);
        assert_eq!(0, count);
        assert_eq!(expected1, step_two);
        let (count, step_three) = increment(&mut step_two);
        assert_eq!(35, count);
        assert_eq!(expected2, step_three);
    }
}
