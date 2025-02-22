//! This module provides a top level
//! [`eframe::App`](https://docs.rs/eframe/0.19.0/eframe/trait.App.html) to contain all variants. A
//! variant must implement [`TTTVariantApp`] to be allowed as a variant.

use crate::{normal::NormalTTTApp, shared::gui::centered_square_in_rect, ultimate::UltimateTTTApp};
use eframe::{
    Storage,
    egui::{self, Context, Ui},
    epaint::{Pos2, Rect},
};

/// This trait represents some variant of tic-tac-toe, wrapped up in a GUI app.
pub trait TTTVariantApp {
    /// Create a new, fresh instance of the app. The storage is passed in to optionally load config.
    fn new_app(storage: Option<&dyn Storage>) -> Self
    where
        Self: Sized;

    /// Show the actual ui of the app. This method is equivalent to
    /// [`eframe::App::update`](https://docs.rs/eframe/0.19.0/eframe/trait.App.html#tymethod.update).
    fn show_ui(&mut self, ctx: &Context, _frame: &mut eframe::Frame);

    /// Save the configuration of the app. This method does nothing by default and is equivalent to
    /// [`eframe::App::save`](https://docs.rs/eframe/0.19.0/eframe/trait.App.html#method.save).
    fn save_config(&mut self, _storage: &mut dyn Storage) {}
}

/// This is the top level wrapper app that contains the variants.
pub struct TTTApp {
    /// This is the variant currently being played. If it's [`None`], then the app will show a
    /// selection screen.
    variant_app: Option<Box<dyn TTTVariantApp>>,
}

impl TTTApp {
    /// Create a new wrapper app with no initial variant app.
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self { variant_app: None }
    }
}

impl eframe::App for TTTApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        match &mut self.variant_app {
            Some(app) => app.show_ui(ctx, frame),
            None => {
                use eframe::epaint::text::{FontFamily, FontId};
                use egui::style::TextStyle::Button as ButtonTextStyle;

                ctx.request_repaint();

                egui::CentralPanel::default().show(ctx, |ui| {
                    // Make the button font size bigger
                    let mut style = (*ctx.style()).clone();
                    let original_button_font = style.text_styles.get(&ButtonTextStyle).cloned();

                    style
                        .text_styles
                        .insert(ButtonTextStyle, FontId::new(30., FontFamily::Proportional));
                    ui.set_style(style);

                    // We only want to use a square in the middle for the buttons
                    let rect = centered_square_in_rect(ui.clip_rect(), 0.7);

                    ui.put(rect, |ui: &mut Ui| {
                        ui.allocate_ui_at_rect(rect, |ui| {
                            // We place the buttons in the top and bottom two fifths of the rect
                            let Pos2 { x: min_x, y: min_y } = rect.min;
                            let Pos2 { x: max_x, y: max_y } = rect.max;
                            let two_fifths = 0.4 * rect.height();

                            if ui
                                .put(
                                    Rect::from_two_pos(
                                        rect.min,
                                        Pos2 {
                                            x: max_x,
                                            y: min_y + two_fifths,
                                        },
                                    ),
                                    egui::Button::new("Normal"),
                                )
                                .clicked()
                            {
                                self.variant_app =
                                    Some(Box::new(NormalTTTApp::new_app(frame.storage())));
                            } else if ui
                                .put(
                                    Rect::from_two_pos(
                                        Pos2 {
                                            x: min_x,
                                            y: max_y - two_fifths,
                                        },
                                        rect.max,
                                    ),
                                    egui::Button::new("Ultimate"),
                                )
                                .clicked()
                            {
                                self.variant_app =
                                    Some(Box::new(UltimateTTTApp::new_app(frame.storage())));
                            }
                        })
                        .response
                    });

                    // Reset the button font size
                    if let Some(id) = original_button_font {
                        let mut style = (*ctx.style()).clone();
                        style.text_styles.insert(ButtonTextStyle, id);
                        ui.set_style(style);
                    }
                });
            }
        }
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        if let Some(app) = &mut self.variant_app {
            app.save_config(storage);
        }
    }
}
