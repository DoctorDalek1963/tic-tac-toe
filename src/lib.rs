//! This crate models [`normal`] and [`ultimate`] games of tic-tac-toe.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod normal;
pub mod ultimate;

pub mod app;
pub mod shared;

#[cfg(target_arch = "wasm32")]
mod fake_par_iter;

#[cfg(test)]
mod test_utils;
