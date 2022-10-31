//! This module handles the local and global boards and the AI player.
//!
//! In a game of [ultimate tic-tac-toe](https://en.wikipedia.org/wiki/Ultimate_tic-tac-toe), there
//! is one global board. This global board is a 3x3 grid of local boards, each of which is a 3x3
//! grid of cells.

use crate::shared::CellShape;

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
}

impl LocalBoard {
    /// Create a new, empty local board.
    pub fn new() -> Self {
        Self {
            cells: [[None; 3]; 3],
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
    pub ai_shape: CellShape,
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
        }
    }
}
