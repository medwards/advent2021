use anyhow::Result;

pub const INPUT_PATH: &str = "inputs/day/9/input";

pub fn part_one(contents: &str) -> Result<usize> {
    Ok(calculate_risk_level(load_height_map(contents)?.as_slice()) as usize)
}

pub fn part_two(contents: &str) -> Result<usize> {
    let height_map = load_height_map(contents)?;
    let mut basins = calculate_basins(
        height_map.as_slice(),
        low_points(height_map.as_slice()).as_slice(),
    );
    basins.sort_unstable();
    basins.reverse();

    Ok(basins.iter().take(3).product())
}

fn calculate_risk_level(height_map: &[Vec<u32>]) -> u32 {
    low_points(height_map)
        .iter()
        .map(|(_, _, height)| height + 1)
        .sum()
}

fn calculate_basins(height_map: &[Vec<u32>], low_points: &[(usize, usize, u32)]) -> Vec<usize> {
    low_points
        .iter()
        .map(|(x, y, _)| {
            let mut basin_points = vec![(*x, *y)];
            loop {
                let old_length = basin_points.len();

                let new_basin_points: Vec<(usize, usize)> = basin_points
                    .iter()
                    .flat_map(|(x_basin, y_basin)| {
                        let stuff: Vec<(usize, usize)> =
                            get_adjacent_points_unchecked(height_map, *x_basin, *y_basin)
                                .iter()
                                .filter(|(x_a, y_a)| {
                                    get_height_unchecked(height_map, *x_a, *y_a) < 9
                                })
                                .map(|(x, y)| (*x, *y))
                                .collect(); // TODO: shouldn't need this
                        stuff
                    })
                    .collect(); // TODO: also shouldn't need this
                basin_points.extend(new_basin_points);
                basin_points.sort_unstable();
                basin_points.dedup();
                if old_length == basin_points.len() {
                    break;
                }
            }
            basin_points.len()
        })
        .collect()
}

fn get_height_unchecked(height_map: &[Vec<u32>], x: usize, y: usize) -> u32 {
    *height_map
        .get(y)
        .expect("Unexpected row index")
        .get(x)
        .expect("unexpected col index")
}

fn get_adjacent_points_unchecked(
    height_map: &[Vec<u32>],
    x: usize,
    y: usize,
) -> Vec<(usize, usize)> {
    let mut adjacent_points = vec![];
    if y > 0 {
        adjacent_points.push((x, y - 1));
    }
    if x > 0 {
        adjacent_points.push((x - 1, y));
    }
    if y < height_map.len() - 1 {
        adjacent_points.push((x, y + 1));
    }
    if x < height_map
        .get(y)
        .expect("column index larger than expected")
        .len()
        - 1
    {
        adjacent_points.push((x + 1, y));
    }
    adjacent_points
}

fn low_points(height_map: &[Vec<u32>]) -> Vec<(usize, usize, u32)> {
    height_map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .flat_map(move |(x, height)| {
                    // SAFETY: vector indices here are sourced from `enumerate` and extra bounds checking
                    // is done in the first boolean clause
                    unsafe {
                        if y > 0 && height_map.get_unchecked(y - 1).get_unchecked(x) <= height {
                            return None;
                        }
                        if x > 0 && height_map.get_unchecked(y).get_unchecked(x - 1) <= height {
                            return None;
                        }
                        if y < height_map.len() - 1
                            && height_map.get_unchecked(y + 1).get_unchecked(x) <= height
                        {
                            return None;
                        }
                        if x < height_map.get_unchecked(y).len() - 1
                            && height_map.get_unchecked(y).get_unchecked(x + 1) <= height
                        {
                            return None;
                        }
                    }
                    Some((x, height))
                })
                .map(move |(x, height)| (x, y, *height))
        })
        .collect()
}

fn load_height_map(contents: &str) -> Result<Vec<Vec<u32>>> {
    contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .ok_or_else(|| anyhow::anyhow!("non-numerical value in heightmap"))
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::read_to_string;

    #[test]
    fn test_calculate_basins() {
        let input = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];

        assert_eq!(
            vec![3, 9, 14, 9],
            calculate_basins(input.as_slice(), low_points(input.as_slice()).as_slice())
        );
    }

    #[test]
    fn test_calculate_risk_level() {
        let input = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];

        assert_eq!(15, calculate_risk_level(input.as_slice()));
    }

    #[test]
    fn test_low_points() {
        let input = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];
        let expected = vec![(1, 0, 1), (9, 0, 0), (2, 2, 5), (6, 4, 5)];

        assert_eq!(expected, low_points(input.as_slice()));
    }

    #[test]
    fn test_load_height_map() {
        let expected = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];
        assert_eq!(
            expected,
            load_height_map(read_to_string("fixtures/heightmap.txt").unwrap().as_str()).unwrap()
        );
    }
}
