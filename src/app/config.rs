use super::TicTacToeApp;
use crate::board::CellShape;
use eframe::egui::{self, Context};
use serde::{Deserialize, Serialize};

/// A struct representing the app configuration, meant to be saved and loaded between sessions.
#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(default)]
pub(crate) struct Config {
    /// Whether the player should make the first move.
    pub(crate) player_plays_first: bool,

    /// Which shape the player uses.
    pub(crate) player_shape: CellShape,

    /// Whether the player is playing against an AI.
    pub(crate) playing_ai: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            player_plays_first: true,
            player_shape: CellShape::X,
            playing_ai: true,
        }
    }
}

impl TicTacToeApp {
    pub(crate) fn draw_settings_window(&mut self, ctx: &Context) {
        egui::Window::new("Settings")
            .collapsible(false)
            .resizable(false)
            .open(&mut self.showing_settings_window)
            .show(ctx, |ui| {
                let mut style = (*ctx.style()).clone();
                for id in style.text_styles.values_mut() {
                    id.size *= 1.2;
                }
                ui.set_style(style);

                ui.checkbox(&mut self.config.playing_ai, "Play against AI");
                ui.add_enabled(
                    self.config.playing_ai,
                    egui::Checkbox::new(&mut self.config.player_plays_first, "Player plays first"),
                );
                ui.horizontal(|ui| {
                    ui.label(if self.config.playing_ai {
                        "Player shape"
                    } else {
                        "First player shape"
                    });
                    ui.radio_value(&mut self.config.player_shape, CellShape::X, "X");
                    ui.radio_value(&mut self.config.player_shape, CellShape::O, "O");
                });
                ui.small("Changes will require a game restart.");
            });
    }
}
