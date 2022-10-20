#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod board;

#[cfg(test)]
mod test_utils;

use crate::app::TicTacToeApp;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Tic-tac-toe",
        options,
        Box::new(|_cc| Box::new(TicTacToeApp::default())),
    );
}
