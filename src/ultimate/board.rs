//! This module handles the local and global boards and the AI player.
//!
//! In a game of [ultimate tic-tac-toe](https://en.wikipedia.org/wiki/Ultimate_tic-tac-toe), there
//! is one global board. This global board is a 3x3 grid of local boards, each of which is a 3x3
//! grid of cells.

use super::GlobalCoord;
use crate::shared::{self, CellShape, WinnerError};
use thiserror::Error;

/// An enum to represent possible errors arising from making a move. See [`GlobalBoard::make_move`].
#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
pub(crate) enum MoveError {
    /// A move has been made in a local board which is not the
    /// [`next_local_board`](GlobalBoard::next_local_board).
    #[error("wrong local board")]
    WrongLocalBoard,

    /// The chosen cell already has a shape in it.
    #[error("cell already full")]
    CellAlreadyFull,

    /// The given coordinate is out of bounds.
    #[error("coordinate out of bounds")]
    OutOfBounds,
}

/// A struct to represent a simple local board with a grid of cells.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LocalBoard {
    /// This 2D array represents all the cells, and is indexed as `cells[x][y]`, with the layout as so:
    ///
    /// ```text
    /// (0, 0) | (1, 0) | (2, 0)
    /// ------------------------
    /// (0, 1) | (1, 1) | (2, 1)
    /// ------------------------
    /// (0, 2) | (1, 2) | (2, 2)
    /// ```
    pub cells: [[Option<CellShape>; 3]; 3],

    /// The winner of the board, used to cache the first winner so that multiple winners can't occur.
    winner: Option<(CellShape, [(usize, usize); 3])>,
}

impl Default for LocalBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalBoard {
    /// Create a new, empty local board.
    pub fn new() -> Self {
        Self {
            cells: [[None; 3]; 3],
            winner: None,
        }
    }

    /// Check if the board is full.
    #[inline(always)]
    fn is_board_full(&self) -> bool {
        shared::is_board_full(self.cells)
    }

    /// Return the winner of the current board. See
    /// [`shared::get_winner`](crate::shared::get_winner).
    pub fn get_winner(&mut self) -> Result<(CellShape, [(usize, usize); 3]), WinnerError> {
        match self.winner {
            None => {
                let winner = shared::get_winner(self.cells)?;
                self.winner = Some(winner);
                Ok(winner)
            }
            Some(x) => Ok(x),
        }
    }
}

/// A struct to represent the global board, with a grid of [`LocalBoard`]s.
#[derive(Clone, Debug, PartialEq)]
pub struct GlobalBoard {
    /// This 2D array represents all the [`LocalBoard`]s, and is indexed as `cells[x][y]`, with the layout as so:
    ///
    /// ```text
    /// (0, 0) | (1, 0) | (2, 0)
    /// ------------------------
    /// (0, 1) | (1, 1) | (2, 1)
    /// ------------------------
    /// (0, 2) | (1, 2) | (2, 2)
    /// ```
    pub local_boards: [[LocalBoard; 3]; 3],

    /// This is the shape that the AI will play as.
    ///
    /// Board positions where this shape wins are considered good, and positions where the other
    /// shape wins are considered bad.
    ai_shape: CellShape,

    /// The local board in which the next move must be played.
    next_local_board: Option<(usize, usize)>,
}

impl Default for GlobalBoard {
    fn default() -> Self {
        Self::new(CellShape::O)
    }
}

impl GlobalBoard {
    /// Create a new, empty global board.
    pub fn new(ai_shape: CellShape) -> Self {
        Self {
            local_boards: [[LocalBoard::new(); 3]; 3],
            ai_shape,
            next_local_board: None,
        }
    }

    /// Return the coordinates of the local board in which the next move must be played.
    pub(crate) fn next_local_board(&self) -> Option<(usize, usize)> {
        self.next_local_board
    }

    /// Check if the given local board has won
    pub(crate) fn has_local_board_won(
        &mut self,
        x: usize,
        y: usize,
    ) -> Result<(CellShape, [(usize, usize); 3]), WinnerError> {
        self.local_boards[x][y].get_winner()
    }

    /// Update the board to reflect a move being made.
    ///
    /// This method will also update the [`next_local_board`](Self::next_local_board), setting it
    /// to [`None`] if the target board is full.
    pub(crate) fn make_move(
        &mut self,
        coord: GlobalCoord,
        shape: CellShape,
    ) -> Result<(), MoveError> {
        let (x, y, (lx, ly)) = coord;

        if x > 2 || y > 2 || lx > 2 || ly > 2 {
            return Err(MoveError::OutOfBounds);
        }

        if let Some(coord) = self.next_local_board {
            if coord != (x, y) {
                return Err(MoveError::WrongLocalBoard);
            }
        }

        let lb = &mut self.local_boards[x][y];
        if lb.cells[lx][ly].is_some() {
            return Err(MoveError::CellAlreadyFull);
        }

        lb.cells[lx][ly] = Some(shape);
        if self.local_boards[lx][ly].is_board_full() {
            self.next_local_board = None;
        } else {
            self.next_local_board = Some((lx, ly));
        }

        Ok(())
    }

    /// Return the winner of the global board. See
    /// [`shared::get_winner`](crate::shared::get_winner).
    pub fn get_winner(&mut self) -> Result<(CellShape, [(usize, usize); 3]), WinnerError> {
        let cells: [[Option<CellShape>; 3]; 3] = self.local_boards.map(|arr| {
            arr.map(|mut board| board.get_winner().map_or(None, |(shape, _)| Some(shape)))
        });

        let result = shared::get_winner(cells);
        if result.is_ok() {
            self.next_local_board = None;
        }
        result
    }
}

#[cfg(test)]
impl LocalBoard {
    pub(crate) fn with_cells(cells: [[Option<CellShape>; 3]; 3]) -> Self {
        Self {
            cells,
            ..Default::default()
        }
    }
}

#[cfg(test)]
impl GlobalBoard {
    /// Create a global board with the given array of local boards. Used in test macros.
    pub(crate) fn with_local_boards(local_boards: [[LocalBoard; 3]; 3]) -> Self {
        Self {
            local_boards,
            ..Default::default()
        }
    }

    /// Create a global board with the given array of local boards and the next local board.
    /// Used in test macros.
    pub(crate) fn with_local_boards_and_next_local_board(
        next_local_board: Option<(usize, usize)>,
        local_boards: [[LocalBoard; 3]; 3],
    ) -> Self {
        Self {
            local_boards,
            next_local_board,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    mod local_board {
        use super::super::*;

        #[test]
        fn is_board_full_test() {
            let mut board = LocalBoard::new();
            assert!(!board.is_board_full());
            board.cells[0][0] = Some(CellShape::X);
            assert!(!board.is_board_full());
            board.cells[1][0] = Some(CellShape::O);
            assert!(!board.is_board_full());
            board.cells[2][0] = Some(CellShape::X);
            assert!(!board.is_board_full());
            board.cells[0][1] = Some(CellShape::O);
            assert!(!board.is_board_full());
            board.cells[1][1] = Some(CellShape::X);
            assert!(!board.is_board_full());
            board.cells[2][1] = Some(CellShape::O);
            assert!(!board.is_board_full());
            board.cells[0][2] = Some(CellShape::X);
            assert!(!board.is_board_full());
            board.cells[1][2] = Some(CellShape::O);
            assert!(!board.is_board_full());
            board.cells[2][2] = Some(CellShape::X);
            assert!(board.is_board_full());
        }
    }

    mod global_board {
        use super::super::*;

        #[test]
        fn make_move_test() {
            let mut board = GlobalBoard::default();

            assert!(board.make_move((1, 1, (0, 0)), CellShape::X).is_ok());
            assert!(
                board.next_local_board == Some((0, 0))
                    && board.local_boards[1][1].cells[0][0] == Some(CellShape::X)
            );

            assert!(
                board.make_move((1, 1, (1, 1)), CellShape::O) == Err(MoveError::WrongLocalBoard)
            );

            assert!(board.make_move((0, 0, (1, 2)), CellShape::O).is_ok());
            assert!(
                board.next_local_board == Some((1, 2))
                    && board.local_boards[0][0].cells[1][2] == Some(CellShape::O)
            );

            assert!(board.make_move((1, 2, (1, 1)), CellShape::X).is_ok());
            assert!(
                board.next_local_board == Some((1, 1))
                    && board.local_boards[1][2].cells[1][1] == Some(CellShape::X)
            );

            assert!(
                board.make_move((1, 1, (0, 0)), CellShape::O) == Err(MoveError::CellAlreadyFull)
            );
            assert!(board.make_move((1, 1, (0, 3)), CellShape::O) == Err(MoveError::OutOfBounds));

            assert!(board.make_move((1, 1, (0, 1)), CellShape::O).is_ok());
            assert!(
                board.next_local_board == Some((0, 1))
                    && board.local_boards[1][1].cells[0][1] == Some(CellShape::O)
            );
        }
    }
}
