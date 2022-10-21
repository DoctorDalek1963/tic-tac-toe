//! This module handles the board and the AI player.

use rayon::prelude::*;
use thiserror::Error;

/// An enum for the shape of a cell on the [`Board`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CellShape {
    X,
    O,
}

impl CellShape {
    /// Return the opposite of the current shape.
    pub fn other(&self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}

/// A possible error that could occur when trying to find a winner,
#[derive(Debug, Error, PartialEq)]
pub enum WinnerError {
    /// Neither player has won, but the board is not full, so a win could occur.
    #[error("Neither player has won yet")]
    NoWinnerYet,

    /// The board is full and neither player won.
    #[error("Board is full but no-one has won")]
    BoardFullNoWinner,

    /// Both players have won.
    ///
    /// This state should never be achievable in normal play, but we need to handle the case where
    /// multiple winning triplets are found in [`Board::get_winner`].
    #[error("Both players have won")]
    MultipleWinners,
}

/// A struct to represent a simple tic-tac-toe board.
#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    /// This 2D array represents all the cells, and is indexed as `cells[x][y]`, with the layout as so:
    ///
    /// (0, 0) | (1, 0) | (2, 0)
    /// ------------------------
    /// (0, 1) | (1, 1) | (2, 1)
    /// ------------------------
    /// (0, 2) | (1, 2) | (2, 2)
    pub cells: [[Option<CellShape>; 3]; 3],

    /// This is the shape that the AI will play as.
    ///
    /// Board positions where this shape wins are considered good, and positions where the other
    /// shape wins are considered bad.
    pub ai_shape: CellShape,
}

impl Board {
    /// Create a new, empty board.
    pub fn new(shape_to_maximise: CellShape) -> Self {
        Self {
            cells: [[None; 3]; 3],
            ai_shape: shape_to_maximise,
        }
    }

    /// Check if the board is full.
    ///
    /// This method does not check for a winner. See [`get_winner`](Board::get_winner).
    fn is_board_full(&self) -> bool {
        self.cells
            .iter()
            .flatten()
            .filter(|cell| cell.is_some())
            .count()
            == 9
    }

    /// Return the winner in the current board position, or a variant of [`WinnerError`] if there is no winner.
    fn get_winner(&self) -> Result<CellShape, WinnerError> {
        let triplets: [[Option<CellShape>; 3]; 8] = [
            [self.cells[0][0], self.cells[0][1], self.cells[0][2]], // Column 0
            [self.cells[1][0], self.cells[1][1], self.cells[1][2]], // Column 1
            [self.cells[2][0], self.cells[2][1], self.cells[2][2]], // Column 2
            [self.cells[0][0], self.cells[1][0], self.cells[2][0]], // Row 0
            [self.cells[0][1], self.cells[1][1], self.cells[2][1]], // Row 1
            [self.cells[0][2], self.cells[1][2], self.cells[2][2]], // Row 2
            [self.cells[0][2], self.cells[1][1], self.cells[2][0]], // +ve diagonal
            [self.cells[0][0], self.cells[1][1], self.cells[2][2]], // -ve diagonal
        ];

        let states = triplets
            .iter()
            .filter_map(
                // Map the arrays into an Option<CellShape> representing their win
                |x| match x {
                    [Some(CellShape::X), Some(CellShape::X), Some(CellShape::X)] => {
                        Some(CellShape::X)
                    }
                    [Some(CellShape::O), Some(CellShape::O), Some(CellShape::O)] => {
                        Some(CellShape::O)
                    }
                    _ => None,
                },
            )
            .collect::<Vec<_>>();

        if states.len() > 1 {
            Err(WinnerError::MultipleWinners)
        } else {
            match states.get(0) {
                None => {
                    if self.is_board_full() {
                        Err(WinnerError::BoardFullNoWinner)
                    } else {
                        Err(WinnerError::NoWinnerYet)
                    }
                }
                Some(x) => Ok(*x),
            }
        }
    }

    /// Return a vector of the coordinates of empty cells in the board.
    ///
    /// This method searches columns before rows.
    fn empty_cells(&self) -> Vec<(usize, usize)> {
        self.cells
            .iter()
            .enumerate()
            .flat_map(|(col, row_vec)| {
                row_vec
                    .iter()
                    .enumerate()
                    .filter_map(|(row, val)| match val {
                        None => Some((col, row)),
                        Some(_) => None,
                    })
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect()
    }

    /// Evaluate the current position of the board, with the context of which shape is playing next.
    ///
    /// Positive numbers are always good for the AI; negative numbers are always good for the player.
    ///
    /// A win for the AI shape is 100. A win for the opponent is -100. For any other position, we
    /// iterate over all possible moves and evaluate each of them, swapping the shape for each
    /// recursion. We also multiple the result of the recursive call by 0.9. This means that
    /// creating or blocking a win in the short term is prioritised over long term play.
    pub fn evaluate_position(&self, shape_to_play: CellShape) -> i8 {
        match self.get_winner() {
            Ok(x) if x == self.ai_shape => 100,
            Ok(x) if x == self.ai_shape.other() => -100,
            Ok(_) => unreachable!(),
            Err(WinnerError::MultipleWinners | WinnerError::BoardFullNoWinner) => 0,
            Err(WinnerError::NoWinnerYet) => {
                let empty_cells = self.empty_cells();

                let map = empty_cells.par_iter().map(|&(x, y)| -> i8 {
                    let mut new_board = self.clone();
                    new_board.cells[x][y] = Some(shape_to_play);
                    // Further moves after this one are considered less important than creating or
                    // blocking a win in the short term
                    (0.9 * new_board.evaluate_position(shape_to_play.other()) as f32) as i8
                });

                if shape_to_play == self.ai_shape {
                    map.max()
                        .expect("We should never iterate over zero empty cells")
                } else {
                    map.min()
                        .expect("We should never iterate over zero empty cells")
                }
            }
        }
    }

    /// Return the optimal position for the AI to play in.
    ///
    /// The optimal move is generated by looking at all possible moves and evaluating each
    /// resultant position, and picking the move which generates the best outcome for the AI.
    /// See [`evaluate_position`](Board::evaluate_position).
    pub fn generate_ai_move(&self) -> (usize, usize) {
        if self.empty_cells().is_empty() {
            todo!("return an error when board is full");
        }

        self.empty_cells()
            .par_iter()
            .map(|&(x, y)| -> ((usize, usize), i8) {
                let mut new_board = self.clone();
                new_board.cells[x][y] = Some(self.ai_shape);
                ((x, y), new_board.evaluate_position(self.ai_shape.other()))
            })
            .max_by_key(|x| x.1)
            .unwrap()
            .0
    }
}

impl Default for Board {
    /// Return a board with [`O`](CellShape::O) as the default AI shape.
    fn default() -> Self {
        Self::new(CellShape::O)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::make_board;

    #[test]
    fn get_winner_test() {
        let board = Board::default();
        assert_eq!(board.get_winner(), Err(WinnerError::NoWinnerYet));

        // X| |
        //  |O|
        //  | |
        let board = make_board!(X E E; E O E; E E E);
        assert_eq!(board.get_winner(), Err(WinnerError::NoWinnerYet));

        // X|O|X
        //  |X|O
        //  |O|X
        let board = make_board!(X O X; E X O; E O X);
        assert_eq!(board.get_winner(), Ok(CellShape::X));

        // O|X|O
        // X|O|X
        // O|X|X
        let board = make_board!(O X O; X O X; O X X);
        assert_eq!(board.get_winner(), Ok(CellShape::O));

        // X|O|O
        // O|X|X
        // X|X|O
        let board = make_board!(X O O; O X X; X X O);
        assert_eq!(board.get_winner(), Err(WinnerError::BoardFullNoWinner));

        // X|X|X
        // O|O|O
        //  | |
        let board = make_board!(X X X; O O O; E E E);
        assert_eq!(board.get_winner(), Err(WinnerError::MultipleWinners));
    }

    #[test]
    fn get_empty_cells_test() {
        let board = Board::default();
        assert_eq!(
            board.empty_cells(),
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (1, 0),
                (1, 1),
                (1, 2),
                (2, 0),
                (2, 1),
                (2, 2),
            ]
        );

        // X|O|X
        //  |X|O
        //  |O|X
        let board = make_board!(X O X; E X O; E O X);
        assert_eq!(board.empty_cells(), vec![(0, 1), (0, 2)]);

        // O|X|O
        // X|O|X
        // O|X|X
        let board = make_board!(O X O; X O X; O X X);
        assert_eq!(board.empty_cells(), vec![]);

        // X|O|O
        // O|X|X
        // X|X|O
        let board = make_board!(X O O; O X X; X X O);
        assert_eq!(board.empty_cells(), vec![]);

        // X|X|X
        // O|O|O
        //  | |
        let board = make_board!(X X X; O O O; E E E);
        assert_eq!(board.empty_cells(), vec![(0, 2), (1, 2), (2, 2)]);
    }

    #[test]
    fn evaluate_position_test() {
        // X|O|
        //  |X|O
        // O| |X
        let board = make_board!(X O E; E X O; O E X);
        // Whoever plays in this position, it's bad because the player (X) has won
        assert_eq!(board.evaluate_position(CellShape::X), -100);
        assert_eq!(board.evaluate_position(CellShape::O), -100);

        // O|X|
        //  |O|X
        // X| |O
        let board = make_board!(O X E; E O X; X E O);
        // Whoever plays in this position, it's good because the AI (O) has won
        assert_eq!(board.evaluate_position(CellShape::X), 100);
        assert_eq!(board.evaluate_position(CellShape::O), 100);

        // X|O|
        // X|O|O
        // X|O|
        let board = make_board!(X O E; X O O; X O E);
        // Multiple winners is a draw
        assert_eq!(board.evaluate_position(CellShape::X), 0);
        assert_eq!(board.evaluate_position(CellShape::O), 0);

        // X|O|
        //  |X|O
        //  | |
        let board = make_board!(X O E; E X E; E E E);
        assert_eq!(board.evaluate_position(CellShape::X), -90);

        // X|O|X
        // X|X|O
        // O| |O
        let board = make_board!(X O X; X X O; O E O);
        assert_eq!(board.evaluate_position(CellShape::X), 0);
        assert_eq!(board.evaluate_position(CellShape::O), 90);

        // X|O|X
        //  |X|O
        // O|X|O
        let board = make_board!(X O X; E X O; O X O);
        assert_eq!(board.evaluate_position(CellShape::X), 0);
        assert_eq!(board.evaluate_position(CellShape::O), 0);
    }

    #[test]
    fn minimax_position_test() {
        //  | |X
        //  |X|O
        //  | |
        let board = make_board!(E E X; E X O; E E E);
        assert_eq!(board.generate_ai_move(), (0, 2));

        // X|O|X
        // X|O|
        //  | |
        let board = make_board!(X O X; X O E; E E E);
        assert_eq!(board.generate_ai_move(), (1, 2));

        //  | |O
        //  |X|
        //  | |X
        let board = make_board!(E E O; E X E; E E X);
        assert_eq!(board.generate_ai_move(), (0, 0));

        // O| |O
        //  |X|
        // X| |X
        let board = make_board!(O E O; E X E; X E X);
        assert_eq!(board.generate_ai_move(), (1, 0));

        // O| |O
        //  |X|
        //  |X|X
        let board = make_board!(O E O; E X E; E X X);
        assert_eq!(board.generate_ai_move(), (1, 0));
    }
}
