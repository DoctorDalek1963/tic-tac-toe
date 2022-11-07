//! This module handles the `egui` interface to the game.

mod config;
mod gui;

use self::config::UltimateConfig;
use super::{board::GlobalBoard, GlobalCoord};
use crate::{app::TTTVariantApp, shared::gui::centered_square_in_rect, CellShape};
use eframe::{egui, epaint::Color32};

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
}

impl Default for UltimateTTTApp {
    fn default() -> Self {
        Self::new_with_config(UltimateConfig::default())
    }
}

impl UltimateTTTApp {
    /// Create a new app with the given config.
    fn new_with_config(config: UltimateConfig) -> Self {
        let global_board = GlobalBoard::new(config.player_shape.other());
        let active_shape = config.player_shape;

        Self {
            config,
            showing_settings_window: false,
            global_board,
            active_shape,
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
