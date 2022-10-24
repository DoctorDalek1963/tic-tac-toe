#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod app;
pub mod board;

#[cfg(target_arch = "wasm32")]
mod fake_par_iter;

#[cfg(test)]
mod test_utils;
