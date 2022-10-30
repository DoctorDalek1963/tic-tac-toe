//! This module models a game of ultimate tic-tac-toe and provides a GUI interface.

/// A coordinate on a local board.
pub type LocalCoord = (usize, usize);

/// A coordinate of an individual cell, as addressed from the global board.
///
/// The first 2 numbers are the `x` and `y` coordinates of the intended local board, and then the
/// [`LocalCoord`] is the coordinate of the desired cell within that local board.
pub type GlobalCoord = (usize, usize, LocalCoord);
