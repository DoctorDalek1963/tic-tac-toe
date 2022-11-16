//! This module handles app configuration.

use super::UltimateTTTApp;
use crate::CellShape;
use eframe::egui::{self, Context};
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        const DEFAULT_MAX_EXPANSIONS: u16 = 1000;
        const DEFAULT_PLAYOUTS: u8 = 1;
        const SLIDER_MAX_EXPANSIONS: u16 = 10_000;
        const SLIDER_MAX_PLAYOUTS: u8 = 25;
    } else {
        const DEFAULT_MAX_EXPANSIONS: u16 = 3000;
        const DEFAULT_PLAYOUTS: u8 = 3;
        const SLIDER_MAX_EXPANSIONS: u16 = 15_000;
        const SLIDER_MAX_PLAYOUTS: u8 = 50;
    }
}

/// A struct representing the app configuration, meant to be saved and loaded between sessions.
#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(default)]
pub struct UltimateConfig {
    /// Whether the player should make the first move.
    pub player_plays_first: bool,

    /// Which shape the player uses.
    pub player_shape: CellShape,

    /// Whether the player is playing against an AI.
    pub playing_ai: bool,

    /// The maximum number of expansions in the AI's MCTS algorithm.
    pub max_mcts_expansions: u16,

    /// The number of playouts to do in each iteration of MCTS.
    pub mcts_playouts: u8,
}

impl Default for UltimateConfig {
    fn default() -> Self {
        Self {
            player_plays_first: true,
            player_shape: CellShape::X,
            playing_ai: false,
            max_mcts_expansions: DEFAULT_MAX_EXPANSIONS,
            mcts_playouts: DEFAULT_PLAYOUTS,
        }
    }
}

impl UltimateTTTApp {
    /// Draw the settings window as a non-collapsible, non-resizable, closable `egui` window.
    pub fn draw_settings_window(&mut self, ctx: &Context) {
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

                if self.config.playing_ai {
                    ui.separator();

                    ui.collapsing("AI Config", |ui| {
                        ui.add(
                            egui::Slider::new(
                                &mut self.config.max_mcts_expansions,
                                1..=SLIDER_MAX_EXPANSIONS,
                            )
                            .clamp_to_range(true)
                            .text("Max expansions in MCTS"),
                        );
                        ui.add(
                            egui::Slider::new(
                                &mut self.config.mcts_playouts,
                                1..=SLIDER_MAX_PLAYOUTS,
                            )
                            .clamp_to_range(true)
                            .text("Number of playouts in each MCTS expansion"),
                        );

                        if ui.button("Reset to defaults").clicked() {
                            self.config = UltimateConfig {
                                max_mcts_expansions: DEFAULT_MAX_EXPANSIONS,
                                mcts_playouts: DEFAULT_PLAYOUTS,
                                ..self.config
                            };
                        }
                    });
                }

                ui.small("Changes will require a game restart.");
            });
    }
}
