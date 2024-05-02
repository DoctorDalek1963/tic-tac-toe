//! This module handles the `egui` interface to the game.

mod config;
mod gui;

use self::config::UltimateConfig;
use super::{board::GlobalBoard, GlobalCoord};
use crate::{app::TTTVariantApp, shared::gui::centered_square_in_rect, CellShape};
use eframe::{egui, epaint::Color32};
use std::sync::mpsc;

/// This method sends an AI-generated move down an `mpsc` channel when it's ready.
#[cfg(not(target_arch = "wasm32"))]
pub fn send_move_when_ready(
    global_board: GlobalBoard,
    max_iters: u16,
    playouts: u8,
    tx: mpsc::Sender<Option<GlobalCoord>>,
) {
    use std::{
        thread,
        time::{Duration, Instant},
    };

    thread::spawn(move || {
        let start = Instant::now();
        let mv = global_board.generate_ai_move(max_iters, playouts);
        thread::sleep(Duration::saturating_sub(
            Duration::from_millis(750),
            start.elapsed(),
        ));
        let _ = tx.send(mv);
    });
}

/// This method sends an AI-generated move down an `mpsc` channel when it's ready.
#[cfg(target_arch = "wasm32")]
pub fn send_move_when_ready(
    global_board: GlobalBoard,
    max_iters: u16,
    playouts: u8,
    tx: mpsc::Sender<Option<GlobalCoord>>,
) {
    use gloo_timers::callback::Timeout;
    use stdweb::web::Date;

    let start = Date::now(); // millis

    Timeout::new(
        u32::saturating_sub(750, (Date::now() - start) as u32),
        move || {
            let _ = tx.send(global_board.generate_ai_move(max_iters, playouts));
        },
    )
    .forget();
}

/// The struct to hold the state of the app.
pub struct UltimateTTTApp {
    /// The configuration of the app.
    config: UltimateConfig,

    /// Whether the settings window is currently being shown.
    showing_settings_window: bool,

    /// The full global board.
    global_board: GlobalBoard,

    /// The shape that will be used for the next cell to be placed.
    ///
    /// See [`update_cell`](UltimateTTTApp::update_cell).
    active_shape: CellShape,

    /// Whether we're currently waiting for the AI to make a move.
    waiting_on_move: bool,

    /// The AI moves are computed in a background thread to make the UI more snappy. This is the
    /// sender that we pass to the background thread to get the AI move back.
    mv_tx: mpsc::Sender<Option<GlobalCoord>>,

    /// The AI moves are computed in a background thread to make the UI more snappy. This is the
    /// receiver that receives the computed AI moves.
    mv_rx: mpsc::Receiver<Option<GlobalCoord>>,
}

impl Default for UltimateTTTApp {
    fn default() -> Self {
        Self::new_with_config(UltimateConfig::default())
    }
}

impl UltimateTTTApp {
    /// Create a new app with the given config.
    ///
    /// If [`UltimateConfig::player_plays_first`] is false, then we also start an AI move in the
    /// background by calling [`send_move_when_ready`].
    fn new_with_config(config: UltimateConfig) -> Self {
        let (mv_tx, mv_rx) = mpsc::channel();

        let global_board = GlobalBoard::new(config.player_shape.other());
        let waiting_on_move = config.playing_ai && !config.player_plays_first;

        let active_shape = if waiting_on_move && config.playing_ai {
            send_move_when_ready(
                global_board.clone(),
                config.max_mcts_expansions,
                config.mcts_playouts,
                mv_tx.clone(),
            );
            config.player_shape.other()
        } else {
            config.player_shape
        };

        Self {
            config,
            showing_settings_window: false,
            global_board,
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
    /// This method uses [`active_shape`](UltimateTTTApp::active_shape) as the shape to place in
    /// the cell and [`GlobalBoard::make_move`] to actually make the move, ignoring any error.
    fn update_cell(&mut self, coord: GlobalCoord) {
        if self
            .global_board
            .make_move(coord, self.active_shape)
            .is_ok()
        {
            self.active_shape = self.active_shape.other();
        }
    }
}

impl TTTVariantApp for UltimateTTTApp {
    fn new_app(storage: Option<&dyn eframe::Storage>) -> Self
    where
        Self: Sized,
    {
        let config = storage.map_or_else(UltimateConfig::default, |storage| {
            eframe::get_value(storage, "ultimate_config").unwrap_or_default()
        });

        Self::new_with_config(config)
    }

    fn show_ui(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
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

            self.draw_global_board(ctx, ui, centered_square_in_rect(ui.clip_rect(), 0.9));
        });

        if self.showing_settings_window {
            self.draw_settings_window(ctx);
        }
    }

    fn save_config(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "ultimate_config", &self.config);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ultimate::test_utils::make_global_board;

    #[test]
    fn update_cell_test() {
        let moves_map: Vec<(GlobalCoord, GlobalBoard)> = vec![
            (
                (1, 1, (1, 1)),
                make_global_board! {
                    next = (1, 1),
                    () () ();
                    () (_; _ X _; _) ();
                    () () ()
                },
            ),
            (
                (1, 1, (0, 0)),
                make_global_board! {
                    next = (0, 0),
                    () () ();
                    () (O _ _; _ X _; _) ();
                    () () ()
                },
            ),
            (
                (0, 0, (0, 1)),
                make_global_board! {
                    next = (0, 1),
                    (_; X _ _; _) () ();
                    () (O _ _; _ X _; _) ();
                    () () ()
                },
            ),
            (
                (0, 1, (1, 1)),
                make_global_board! {
                    next = (1, 1),
                    (_; X _ _; _) () ();
                    (_; _ O _; _) (O _ _; _ X _; _) ();
                    () () ()
                },
            ),
            (
                (1, 1, (0, 2)),
                make_global_board! {
                    next = (0, 2),
                    (_; X _ _; _) () ();
                    (_; _ O _; _) (O _ _; _ X _; X _ _) ();
                    () () ()
                },
            ),
        ];

        let mut app = UltimateTTTApp::default();
        assert_eq!(app.global_board, GlobalBoard::default());

        for (coord, global_board) in moves_map {
            app.update_cell(coord);
            assert_eq!(
                app.global_board, global_board,
                "coord = {:?}; global_board = {:?}",
                coord, global_board
            );
        }
    }
}
