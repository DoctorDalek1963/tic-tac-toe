//! This module models a game of standard tic-tac-toe and provides a GUI interface.

/// A coordinate on the board. See [`Board::cells`](board::Board::cells).
pub type Coord = (usize, usize);

pub mod app;
pub mod board;

pub use self::app::NormalTTTApp;

#[cfg(all(test, not(feature = "bench")))]
pub(crate) mod test_utils;

#[cfg(feature = "bench")]
pub mod test_utils;
