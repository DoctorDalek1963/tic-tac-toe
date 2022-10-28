//! This module handles the `egui` interface to the game.

mod config;
mod gui;

use self::config::Config;
use crate::{
    board::{Board, CellShape},
    Coord,
};
use eframe::{
    egui::{self, Context, Rect},
    epaint::Vec2,
};
use std::sync::mpsc;

type CoordResult = Result<Coord, ()>;

/// Create a centered square in the given rect, taking up the given percentage of length.
fn centered_square_in_rect(rect: Rect, percent: f32) -> Rect {
    let Vec2 { x, y } = rect.max - rect.min;
    let length = percent * x.min(y);

    Rect::from_center_size(rect.center(), Vec2::splat(length))
}

/// This method sends an AI-generated move down an `mpsc` channel after 200ms.
#[cfg(not(target_arch = "wasm32"))]
fn send_move_after_delay(board: Board, tx: mpsc::Sender<CoordResult>) {
    use std::{thread, time::Duration};
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(200));
        let _ = tx.send(board.generate_ai_move());
    });
}

/// This method sends an AI-generated move down an `mpsc` channel after 200ms.
#[cfg(target_arch = "wasm32")]
fn send_move_after_delay(board: Board, tx: mpsc::Sender<CoordResult>) {
    gloo_timers::callback::Timeout::new(200, move || {
        let _ = tx.send(board.generate_ai_move());
    })
    .forget();
}

/// The struct to hold the state of the app.
pub struct TicTacToeApp {
    /// The configuration of the app.
    config: Config,

    /// The actual board itself.
    board: Board,

    /// The shape that will be used for the next cell to be placed.
    ///
    /// See [`update_cell`](TicTacToeApp::update_cell).
    active_shape: CellShape,

    /// Whether we're currently waiting for the AI to make a move.
    waiting_on_move: bool,

    /// The AI moves are computed in a background thread to make the UI more snappy. This is the
    /// sender that we pass to the background thread to get the AI move back.
    mv_tx: mpsc::Sender<CoordResult>,

    /// The AI moves are computed in a background thread to make the UI more snappy. This is the
    /// receiver that receives the computed AI moves.
    mv_rx: mpsc::Receiver<CoordResult>,
}

impl Default for TicTacToeApp {
    fn default() -> Self {
        Self::new_with_config(Config::default())
    }
}

impl TicTacToeApp {
    /// Create a new app, attempting to restore previous [`Config`], or using the default config.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let config = if let Some(storage) = cc.storage {
            eframe::get_value(storage, "config").unwrap_or_default()
        } else {
            Config::default()
        };

        Self::new_with_config(config)
    }

    /// Create a new app with the given config.
    ///
    /// If [`Config::player_plays_first`] is false, then we also start an AI move in the background
    /// by calling [`send_move_after_delay`].
    fn new_with_config(config: Config) -> Self {
        let (mv_tx, mv_rx) = mpsc::channel();

        let board = Board::new(config.player_shape.other());
        let waiting_on_move = !config.player_plays_first;

        let active_shape = if waiting_on_move {
            send_move_after_delay(board.clone(), mv_tx.clone());
            config.player_shape.other()
        } else {
            config.player_shape
        };

        Self {
            config,
            board,
            active_shape,
            waiting_on_move,
            mv_tx,
            mv_rx,
        }
    }

    /// Update the board to reflect a cell being clicked.
    ///
    /// This method uses [`active_shape`](TicTacToeApp::active_shape) as the shape to place in the cell.
    fn update_cell(&mut self, x: usize, y: usize) {
        if x > 2 || y > 2 {
            return;
        }

        if self.board.cells[x][y].is_none() {
            self.board.cells[x][y] = Some(self.active_shape);
            self.active_shape = self.active_shape.other();
        }
    }
}

impl eframe::App for TicTacToeApp {
    /// Show the app itself.
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_board(ctx, ui, centered_square_in_rect(ui.clip_rect(), 0.9));
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "config", &self.config);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::make_board;
    use crate::Coord;

    #[test]
    fn update_cell_test() {
        let map_1: Vec<(Coord, Board)> = vec![
            ((0, 1), make_board!(E E E; X E E; E E E)),
            ((2, 1), make_board!(E E E; X E O; E E E)),
            ((0, 0), make_board!(X E E; X E O; E E E)),
            ((2, 2), make_board!(X E E; X E O; E E O)),
            ((2, 0), make_board!(X E X; X E O; E E O)),
            ((0, 2), make_board!(X E X; X E O; O E O)),
        ];

        let map_2: Vec<(Coord, Board)> = vec![
            ((0, 3), make_board!(E E E; E E E; E E E)),
            ((6, 3), make_board!(E E E; E E E; E E E)),
            ((1, 1), make_board!(E E E; E X E; E E E)),
            ((1, 1), make_board!(E E E; E X E; E E E)),
            ((1, 1), make_board!(E E E; E X E; E E E)),
            ((1, 1), make_board!(E E E; E X E; E E E)),
            ((2, 1), make_board!(E E E; E X O; E E E)),
        ];

        for moves_map in [map_1, map_2] {
            let mut app = TicTacToeApp::default();
            assert_eq!(app.board, Board::default());

            for ((x, y), board) in moves_map {
                app.update_cell(x, y);
                assert_eq!(app.board, board);
            }
        }
    }
}
