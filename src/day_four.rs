use std::{ops::ControlFlow, str::FromStr};

use anyhow::Result;

pub const INPUT_PATH: &str = "inputs/day/4/input";

pub fn part_one(contents: &str) -> Result<usize> {
    Ok(Bingo::from_str(contents)?.play())
}

pub fn part_two(contents: &str) -> Result<usize> {
    Ok(Bingo::from_str(contents)?.play_to_lose())
}

#[derive(Debug, PartialEq)]
struct Bingo {
    balls: Vec<usize>,
    boards: Vec<Board>,
}

impl Bingo {
    fn play(&mut self) -> usize {
        for ball in self.balls.iter() {
            let score = self.boards.iter_mut().fold(0, |score, board| {
                board.mark(*ball);
                if board.is_complete() {
                    score + (board.score() * *ball)
                } else {
                    score
                }
            });
            if score > 0 {
                return score;
            }
        }
        0
    }

    fn play_to_lose(&mut self) -> usize {
        let boards_result =
            self.balls
                .iter()
                .try_fold((self.boards.clone(), 0), |(mut boards, score), ball| {
                    let keep_board = boards.len() == 1;
                    boards.iter_mut().for_each(|board| board.mark(*ball));
                    boards.retain(|board| keep_board || !board.is_complete());
                    if boards.len() == 1 {
                        if let Some(board) = boards.get(0) {
                            if board.is_complete() {
                                let score = board.score() * *ball;
                                return ControlFlow::Break((boards, score));
                            }
                        }
                    }
                    ControlFlow::Continue((boards, score))
                });
        match boards_result {
            ControlFlow::Break((_, score)) => score,
            _ => 0,
        }
    }
}

impl FromStr for Bingo {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut lines = s.lines();
        let balls: Vec<usize> = lines
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing balls line"))?
            .split(',')
            .map(|b| b.parse())
            .collect::<std::result::Result<Vec<_>, _>>()?;

        let bingo = Bingo {
            balls,
            boards: Vec::new(),
        };

        // breaks badly if input isn't good
        let (bingo, _, last_square) = lines.fold(
            (
                bingo,
                Board {
                    squares: [0; 25],
                    marked: [0; 25],
                },
                0,
            ),
            |(mut bingo, mut board, squares_index), line| {
                let mut squares_index = squares_index
                    + line
                        .split(' ')
                        .flat_map(|s| s.parse::<usize>())
                        .enumerate()
                        .map(|(i, val)| {
                            board.squares[squares_index + i] = val;
                        })
                        .count();
                if squares_index == 25 {
                    bingo.boards.push(board.clone());
                    squares_index = 0;
                }
                (bingo, board, squares_index)
            },
        );

        if last_square != 0 {
            return Err(anyhow::anyhow!("Incomplete input"));
        }

        Ok(bingo)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Board {
    squares: [usize; 25],
    marked: [u8; 25],
}

impl Board {
    fn mark(&mut self, value: usize) {
        self.squares.iter().enumerate().for_each(|(i, s)| {
            if *s == value {
                self.marked[i] = 1
            }
        });
    }

    fn is_complete(&self) -> bool {
        let row_score: usize = self
            .marked
            .chunks(5)
            .map(|chunk| if chunk.iter().sum::<u8>() == 5 { 5 } else { 0 })
            .sum();

        if row_score == 5 {
            return true;
        }

        let col_score: usize = (0..5)
            .map(|col| {
                let score: u8 = self.marked.iter().skip(col).step_by(5).sum();
                if score == 5 {
                    5
                } else {
                    0
                }
            })
            .sum();

        if col_score == 5 {
            return true;
        }

        /* WHAT BINGO DOESN'T HONOUR DIAGNOALS?!??!!
        let diag1_score: usize = (0..5).map(|i| self.marked[(i * 5) + i] as usize).sum();

        if diag1_score == 5 {
            return true;
        }

        let diag2_score: usize = (0..5)
            .rev()
            .enumerate()
            .map(|(col, row)| self.marked[(col * 5) + row] as usize)
            .sum();

        if diag2_score == 5 {
            return true;
        }
        */

        false
    }

    fn score(&self) -> usize {
        self.squares
            .iter()
            .enumerate()
            .filter(|(i, _)| self.marked[*i] != 1)
            .map(|(_, value)| value)
            .sum()
    }

    fn display_marked(&self) -> String {
        self.marked
            .iter()
            .enumerate()
            .map(|(i, marked)| {
                if (i + 1) % 5 == 0 {
                    if *marked == 0 {
                        " \n"
                    } else {
                        "X\n"
                    }
                } else if *marked == 0 {
                    " "
                } else {
                    "X"
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_bingo_play() {
        assert_eq!(4512, expected_bingo().play());
    }

    #[test]
    fn test_bingo_play_to_lose() {
        assert_eq!(1924, expected_bingo().play_to_lose());
    }

    #[test]
    fn test_bingo_from_str() {
        let contents = read_to_string("fixtures/bingo.txt").unwrap();

        assert_eq!(
            expected_bingo(),
            Bingo::from_str(contents.as_str()).unwrap()
        );
    }

    #[test]
    fn test_board_is_complete() {
        let incomplete_board = Board {
            squares: [0; 25],
            marked: [
                1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        };

        println!("{}", incomplete_board.display_marked());
        assert!(!incomplete_board.is_complete());

        let completed_row_board = Board {
            squares: [0; 25],
            marked: [
                0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        };

        println!("{}", completed_row_board.display_marked());
        assert!(completed_row_board.is_complete());

        let completed_col_board = Board {
            squares: [0; 25],
            marked: [
                1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0,
            ],
        };

        println!("{}", completed_col_board.display_marked());
        assert!(completed_col_board.is_complete());

        // IMPORTANT: Diagonal boards are NOT complete
        let completed_diag_board1 = Board {
            squares: [0; 25],
            marked: [
                1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1,
            ],
        };

        println!("{}", completed_diag_board1.display_marked());
        assert!(!completed_diag_board1.is_complete());

        let completed_diag_board2 = Board {
            squares: [0; 25],
            marked: [
                0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0,
            ],
        };

        println!("{}", completed_diag_board2.display_marked());
        assert!(!completed_diag_board2.is_complete());
    }

    fn expected_bingo() -> Bingo {
        Bingo {
            balls: vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ],
            boards: vec![
                Board {
                    squares: [
                        22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1,
                        12, 20, 15, 19,
                    ],
                    marked: [0; 25],
                },
                Board {
                    squares: [
                        3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14,
                        21, 16, 12, 6,
                    ],
                    marked: [0; 25],
                },
                Board {
                    squares: [
                        14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5,
                        2, 0, 12, 3, 7,
                    ],
                    marked: [0; 25],
                },
            ],
        }
    }
}
