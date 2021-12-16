use anyhow::Result;

pub const INPUT_PATH: &str = "inputs/day/15/input";

pub fn part_one(contents: &str) -> Result<usize> {
    Ok(min_risk(&load_grid(contents)?))
}

pub fn part_two(contents: &str) -> Result<usize> {
    let grid = load_grid(contents)?;
    Ok(min_risk(&tile_grid(&grid)))
}

fn min_risk(grid: &Vec<Vec<u8>>) -> usize {
    let dimensions = (grid.len(), grid.first().expect("empty grid").len());
    let mut min = vec![vec![None; dimensions.1]; dimensions.0];

    // first pass - will not calculate /w risk paths coming from below or right
    evaluate_min_grid(grid, &mut min);
    // second pass - improve min - are two passes enough?
    evaluate_min_grid(grid, &mut min);

    min.get(grid.len() - 1)
        .unwrap()
        .get(grid.len() - 1)
        .unwrap()
        .unwrap()
}

fn evaluate_min_grid(grid: &Vec<Vec<u8>>, min_grid: &mut Vec<Vec<Option<usize>>>) {
    let dimensions = (grid.len(), grid.first().expect("empty grid").len());
    (0..dimensions.0).for_each(|i| {
        (0..dimensions.1).for_each(|j| {
            if i == 0 && j == 0 {
                *min_grid.get_mut(i).unwrap().get_mut(j).unwrap() = Some(0);
            } else {
                let neighbours = [
                    if i > 0 { Some((i - 1, j)) } else { None },
                    if j > 0 { Some((i, j - 1)) } else { None },
                    if i < dimensions.0 - 1 {
                        Some((i + 1, j))
                    } else {
                        None
                    },
                    if j < dimensions.1 - 1 {
                        Some((i, j + 1))
                    } else {
                        None
                    },
                ];
                let lowest_risk = neighbours
                    .iter()
                    .flat_map(|o| *o) // throw away invalid neighbours
                    .flat_map(|(i, j)| min_grid.get(i).unwrap().get(j).unwrap()) // throw away cells with no min yet
                    .min();
                *min_grid.get_mut(i).unwrap().get_mut(j).unwrap() =
                    Some(lowest_risk.unwrap() + *grid.get(i).unwrap().get(j).unwrap() as usize);
            }
        })
    });
}

// Apparently this is really really slow
// ```
// yay: [(0, 0), (0, 1), (1, 1), (2, 1), (3, 1), (3, 2), (3, 3), (4, 3), (4, 4), (4, 5), (4, 6), (4, 7), (4, 8), (4, 9), (5, 9), (6, 9), (7, 9), (8, 9), (9, 9)] (queue len 6071314)
// ```
//
// That queue len!
/*
fn bfs_find_paths(grid: &Vec<Vec<u8>>) -> Vec<usize> {
    let mut paths = Vec::new();
    let dimensions = (grid.first().expect("incomplete grid").len(), grid.len());
    let mut queue = VecDeque::from(vec![(vec![(0, 0)], 0 as usize)]);
    while !queue.is_empty() {
        let (path, risk) = queue
            .pop_front()
            .expect("empty queue but loop condition was !queue.is_empty()");

        let v = path
            .iter()
            .last()
            .expect("empty path, but path is always initialized with one val");

        if (v.0 == dimensions.0 - 1 && v.1 == dimensions.1 - 2)
            || (v.0 == dimensions.0 - 2 && v.1 == dimensions.1 - 1)
        {
            let mut path = path.clone();
            let v = (dimensions.0 - 1, dimensions.1 - 1);
            path.push(v);
            println!("yay: {:?} (queue len {})", path, queue.len());
            let risk = risk + (grid.as_slice()[v.1].as_slice()[v.0]) as usize;
            paths.push((path, risk));
            continue;
        }

        let new_v = [
            if v.0 > 0 { Some((v.0 - 1, v.1)) } else { None },
            if v.1 > 0 { Some((v.0, v.1 - 1)) } else { None },
            if v.0 < dimensions.0 - 1 {
                Some((v.0 + 1, v.1))
            } else {
                None
            },
            if v.1 < dimensions.1 - 1 {
                Some((v.0, v.1 + 1))
            } else {
                None
            },
        ];

        queue.extend(
            new_v
                .into_iter()
                .flat_map(|m| {
                    m.as_ref().and_then(|coord| {
                        if path.contains(coord) {
                            None
                        } else {
                            Some(*coord)
                        }
                    })
                })
                .map(|(x, y)| {
                    let risk = risk + (grid.as_slice()[y].as_slice()[x]) as usize;
                    let mut path = path.clone();
                    path.push((x, y));
                    (path, risk)
                }),
        );
    }

    paths.iter().map(|(_path, risk)| *risk).collect()
}
*/

fn tile_grid(tile: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let tile_dimensions = (tile.len(), tile.first().expect("empty tile").len());
    let dimensions = (tile_dimensions.0 * 5, tile_dimensions.1 * 5);

    let mut grid = vec![vec![0; dimensions.1]; dimensions.0];
    (0..dimensions.0).for_each(|i| {
        (0..dimensions.1).for_each(|j| {
            let tile_index = ((i / tile_dimensions.0) as u8, (j / tile_dimensions.1) as u8);
            if tile_index == (0, 0) {
                // copy pasta
                *grid.get_mut(i).unwrap().get_mut(j).unwrap() =
                    *tile.get(i).unwrap().get(j).unwrap();
            } else {
                let tile_i = i % tile_dimensions.0;
                let tile_j = j % tile_dimensions.1;
                let val =
                    tile.get(tile_i).unwrap().get(tile_j).unwrap() + tile_index.0 + tile_index.1;
                *grid.get_mut(i).unwrap().get_mut(j).unwrap() = if val > 9 { val - 9 } else { val };
            }
        })
    });

    grid
}

fn load_grid(contents: &str) -> Result<Vec<Vec<u8>>> {
    Ok(contents
        .lines()
        .map(|line| line.as_bytes().iter().map(|b| b - b'0').collect())
        .collect())
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_load_grid() {
        let expected = [
            [1, 1, 6, 3, 7, 5, 1, 7, 4, 2].to_vec(),
            [1, 3, 8, 1, 3, 7, 3, 6, 7, 2].to_vec(),
            [2, 1, 3, 6, 5, 1, 1, 3, 2, 8].to_vec(),
            [3, 6, 9, 4, 9, 3, 1, 5, 6, 9].to_vec(),
            [7, 4, 6, 3, 4, 1, 7, 1, 1, 1].to_vec(),
            [1, 3, 1, 9, 1, 2, 8, 1, 3, 7].to_vec(),
            [1, 3, 5, 9, 9, 1, 2, 4, 2, 1].to_vec(),
            [3, 1, 2, 5, 4, 2, 1, 6, 3, 9].to_vec(),
            [1, 2, 9, 3, 1, 3, 8, 5, 2, 1].to_vec(),
            [2, 3, 1, 1, 9, 4, 4, 5, 8, 1].to_vec(),
        ]
        .to_vec();

        assert_eq!(
            expected,
            load_grid(read_to_string("fixtures/grid.txt").unwrap().as_str()).unwrap()
        );
    }

    #[test]
    fn test_tile_grid() {
        let input = [
            [1, 1, 6, 3, 7, 5, 1, 7, 4, 2].to_vec(),
            [1, 3, 8, 1, 3, 7, 3, 6, 7, 2].to_vec(),
            [2, 1, 3, 6, 5, 1, 1, 3, 2, 8].to_vec(),
            [3, 6, 9, 4, 9, 3, 1, 5, 6, 9].to_vec(),
            [7, 4, 6, 3, 4, 1, 7, 1, 1, 1].to_vec(),
            [1, 3, 1, 9, 1, 2, 8, 1, 3, 7].to_vec(),
            [1, 3, 5, 9, 9, 1, 2, 4, 2, 1].to_vec(),
            [3, 1, 2, 5, 4, 2, 1, 6, 3, 9].to_vec(),
            [1, 2, 9, 3, 1, 3, 8, 5, 2, 1].to_vec(),
            [2, 3, 1, 1, 9, 4, 4, 5, 8, 1].to_vec(),
        ]
        .to_vec();

        let expected =
            load_grid(read_to_string("fixtures/grid_tiled.txt").unwrap().as_str()).unwrap();

        assert_eq!(expected, tile_grid(&input));
    }

    #[test]
    fn test_min_risk() {
        assert_eq!(
            40,
            min_risk(&load_grid(read_to_string("fixtures/grid.txt").unwrap().as_str()).unwrap())
        );
    }
}
