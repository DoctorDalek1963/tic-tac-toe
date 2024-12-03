//! This module handles the `egui` interface to the game.

mod config;
mod gui;

use self::config::NormalConfig;
use super::{board::Board, Coord};
use crate::{app::TTTVariantApp, shared::gui::centered_square_in_rect, CellShape};
use eframe::{
    egui::{self, Context},
    epaint::Color32,
};
use std::sync::mpsc;
use web_time::{Duration, Instant};

/// This method sends an AI-generated move down an `mpsc` channel after 200ms.
#[cfg(not(target_arch = "wasm32"))]
pub fn send_move_after_delay(board: Board, tx: mpsc::Sender<Option<Coord>>) {
    use std::thread;

    thread::spawn(move || {
        let start = Instant::now();
        let mv = board.generate_ai_move();
        thread::sleep(Duration::from_millis(200) - start.elapsed());
        let _ = tx.send(mv);
    });
}

/// This method sends an AI-generated move down an `mpsc` channel after 200ms.
#[cfg(target_arch = "wasm32")]
pub fn send_move_after_delay(board: Board, tx: mpsc::Sender<Option<Coord>>) {
    let start = Instant::now();
    let mv = board.generate_ai_move();

    gloo_timers::callback::Timeout::new(
        (Duration::from_millis(200) - start.elapsed()).as_millis() as u32,
        move || {
            let _ = tx.send(mv);
        },
    )
    .forget();
}

/// The struct to hold the state of the app.
pub struct NormalTTTApp {
    /// The configuration of the app.
    config: NormalConfig,

    /// Whether the settings window is currently being shown.
    showing_settings_window: bool,

    /// The actual board itself.
    board: Board,

    /// The shape that will be used for the next cell to be placed.
    ///
    /// See [`update_cell`](NormalTTTApp::update_cell).
    active_shape: CellShape,

    /// Whether we're currently waiting for the AI to make a move.
    waiting_on_move: bool,

    /// The AI moves are computed in a background thread to make the UI more snappy. This is the
    /// sender that we pass to the background thread to get the AI move back.
    mv_tx: mpsc::Sender<Option<Coord>>,

    /// The AI moves are computed in a background thread to make the UI more snappy. This is the
    /// receiver that receives the computed AI moves.
    mv_rx: mpsc::Receiver<Option<Coord>>,
}

impl Default for NormalTTTApp {
    fn default() -> Self {
        Self::new_with_config(NormalConfig::default())
    }
}

impl NormalTTTApp {
    /// Create a new app with the given config.
    ///
    /// If [`NormalConfig::player_plays_first`] is false, then we also start an AI move in the
    /// background by calling [`send_move_after_delay`].
    fn new_with_config(config: NormalConfig) -> Self {
        let (mv_tx, mv_rx) = mpsc::channel();

        let board = Board::new(config.player_shape.other());
        let waiting_on_move = config.playing_ai && !config.player_plays_first;

        let active_shape = if waiting_on_move && config.playing_ai {
            send_move_after_delay(board.clone(), mv_tx.clone());
            config.player_shape.other()
        } else {
            config.player_shape
        };

        Self {
            config,
            showing_settings_window: false,
            board,
            active_shape,
            waiting_on_move,
            mv_tx,
            mv_rx,
        }
    }

    /// Update the interior state of the app with the current config.
    ///
    /// See [`Self::new_with_config`]
    fn restart_game(&mut self) {
        *self = Self::new_with_config(self.config);
    }

    /// Update the board to reflect a cell being clicked.
    ///
    /// This method uses [`active_shape`](NormalTTTApp::active_shape) as the shape to place in the cell.
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

impl TTTVariantApp for NormalTTTApp {
    fn new_app(storage: Option<&dyn eframe::Storage>) -> Self
    where
        Self: Sized,
    {
        let config = storage.map_or_else(NormalConfig::default, |storage| {
            eframe::get_value(storage, "normal_config").unwrap_or_default()
        });

        Self::new_with_config(config)
    }

    /// Show the app itself.
    fn show_ui(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Show the restart game and settings buttons
            ui.horizontal(|ui| {
                use eframe::epaint::{FontFamily, FontId};
                use egui::TextStyle::Button;

                let mut style = (*ctx.style()).clone();
                style
                    .text_styles
                    .insert(Button, FontId::new(30., FontFamily::Proportional));
                ui.set_style(style);

                if ui
                    .add(egui::Button::new("\u{27F3}").fill(Color32::TRANSPARENT))
                    .clicked()
                {
                    self.restart_game();
                }

                if ui
                    .add(
                        egui::Button::new("\u{2699}").fill(if self.showing_settings_window {
                            if ctx.style().visuals.dark_mode {
                                Color32::from_rgb(0x00, 0x5C, 0x80)
                            } else {
                                Color32::from_rgb(0x90, 0xD1, 0xFF)
                            }
                        } else {
                            Color32::TRANSPARENT
                        }),
                    )
                    .clicked()
                {
                    self.showing_settings_window = !self.showing_settings_window;
                }
            });

            self.draw_board(ctx, ui, centered_square_in_rect(ui.clip_rect(), 0.9));
        });

        if self.showing_settings_window {
            self.draw_settings_window(ctx);
        }
    }

    fn save_config(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "normal_config", &self.config);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::normal::test_utils::make_board;
    use crate::normal::Coord;

    #[test]
    fn update_cell_test() {
        let map_1: Vec<(Coord, Board)> = vec![
            ((0, 1), make_board!(_; X _ _; _)),
            ((2, 1), make_board!(_; X _ O; _)),
            ((0, 0), make_board!(X _ _; X _ O; _)),
            ((2, 2), make_board!(X _ _; X _ O; _ _ O)),
            ((2, 0), make_board!(X _ X; X _ O; _ _ O)),
            ((0, 2), make_board!(X _ X; X _ O; O _ O)),
        ];

        let map_2: Vec<(Coord, Board)> = vec![
            ((0, 3), make_board!(_; _; _)),
            ((6, 3), make_board!(_; _; _)),
            ((1, 1), make_board!(_; _ X _; _)),
            ((1, 1), make_board!(_; _ X _; _)),
            ((1, 1), make_board!(_; _ X _; _)),
            ((1, 1), make_board!(_; _ X _; _)),
            ((2, 1), make_board!(_; _ X O; _)),
        ];

        for moves_map in [map_1, map_2] {
            let mut app = NormalTTTApp::default();
            assert_eq!(app.board, Board::default());

            for ((x, y), board) in moves_map {
                app.update_cell(x, y);
                assert_eq!(app.board, board);
            }
        }
    }
}
