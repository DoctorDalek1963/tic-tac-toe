//! This crate models [`normal`] and [`ultimate`] games of tic-tac-toe.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod normal;
pub mod ultimate;

pub mod app;
pub mod shared;

pub use self::shared::board::CellShape;

#[cfg(target_arch = "wasm32")]
mod fake_par_iter;

#[cfg(all(test, not(feature = "bench")))]
pub(crate) mod test_utils;

#[cfg(feature = "bench")]
pub mod test_utils;
