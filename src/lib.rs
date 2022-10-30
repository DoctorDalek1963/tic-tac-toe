//! This crate models a game of standard tic-tac-toe and provides a GUI interface.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// A coordinate on the board. See [`Board::cells`](crate::board::Board::cells).
pub type Coord = (usize, usize);

pub mod app;
pub mod board;

#[cfg(target_arch = "wasm32")]
mod fake_par_iter;

#[cfg(test)]
mod test_utils;
