#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod app;
pub mod board;

#[cfg(test)]
mod test_utils;
