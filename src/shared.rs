//! This module provides various types that are shared between variants.

use eframe::epaint::{Rect, Vec2};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// An enum for the shape of a cell on the [`Board`].
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
    /// multiple winning triplets are found in [`Board::get_winner`].
    #[error("Both players have won")]
    MultipleWinners,
}

/// Create a centered square in the given rect, taking up the given percentage of length.
pub fn centered_square_in_rect(rect: Rect, percent: f32) -> Rect {
    let Vec2 { x, y } = rect.max - rect.min;
    let length = percent * x.min(y);

    Rect::from_center_size(rect.center(), Vec2::splat(length))
}
