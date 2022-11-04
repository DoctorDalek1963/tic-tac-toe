//! This module provides types and functions that are shared between variant backends.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// An enum for the shape of a cell on the board.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum CellShape {
    X,
    O,
}

impl CellShape {
    /// Return the opposite of the current shape.
    #[must_use]
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
    /// multiple winning triplets are found in `get_winner()` methods for variant boards.
    #[error("Both players have won")]
    MultipleWinners,
}

/// Check if the board is full.
///
/// This method does not check for a winner. See [`get_winner`](Board::get_winner).
pub fn is_board_full(cells: [[Option<CellShape>; 3]; 3]) -> bool {
    cells.iter().flatten().filter(|cell| cell.is_some()).count() == 9
}

/// Return the winner in the current board position, or a variant of [`WinnerError`] if there is no winner.
///
/// # Errors
///
/// - [`NoWinnerYet`](WinnerError::NoWinnerYet): There is currently no winner, but there could be
/// in the future.
/// - [`BoardFullNoWinner`](WinnerError::BoardFullNoWinner): The board is full and neither player
/// has won.
/// - [`MultipleWinners`](WinnerError::MultipleWinners): Both players have won. This should never
/// be achievable in normal play.
pub fn get_winner(
    cells: [[Option<CellShape>; 3]; 3],
) -> Result<(CellShape, [(usize, usize); 3]), WinnerError> {
    // This closure returns a tuple with the shapes and the actual coordinates
    let get_triplet =
        |coords: [(usize, usize); 3]| -> ([Option<CellShape>; 3], [(usize, usize); 3]) {
            let get_cell = |coord: (usize, usize)| -> Option<CellShape> { cells[coord.0][coord.1] };

            (
                [
                    get_cell(coords[0]),
                    get_cell(coords[1]),
                    get_cell(coords[2]),
                ],
                coords,
            )
        };

    // Each element of this array is a tuple of the shapes and the actual coordinates
    let triplets: [([Option<CellShape>; 3], [(usize, usize); 3]); 8] = [
        get_triplet([(0, 0), (0, 1), (0, 2)]), // Column 0
        get_triplet([(1, 0), (1, 1), (1, 2)]), // Column 1
        get_triplet([(2, 0), (2, 1), (2, 2)]), // Column 2
        get_triplet([(0, 0), (1, 0), (2, 0)]), // Row 0
        get_triplet([(0, 1), (1, 1), (2, 1)]), // Row 1
        get_triplet([(0, 2), (1, 2), (2, 2)]), // Row 2
        get_triplet([(0, 2), (1, 1), (2, 0)]), // +ve diagonal
        get_triplet([(0, 0), (1, 1), (2, 2)]), // -ve diagonal
    ];

    let states: Vec<(CellShape, [(usize, usize); 3])> = triplets
        .iter()
        .filter_map(
            // Map the arrays into an Option<CellShape> representing their win
            |&(shapes, coords)| match shapes {
                [Some(CellShape::X), Some(CellShape::X), Some(CellShape::X)] => {
                    Some((CellShape::X, coords))
                }
                [Some(CellShape::O), Some(CellShape::O), Some(CellShape::O)] => {
                    Some((CellShape::O, coords))
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
                if is_board_full(cells) {
                    Err(WinnerError::BoardFullNoWinner)
                } else {
                    Err(WinnerError::NoWinnerYet)
                }
            }
            Some(x) => Ok(*x),
        }
    }
}
