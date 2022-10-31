//! This module provides a top level
//! [`eframe::App`](https://docs.rs/eframe/0.19.0/eframe/trait.App.html) to contain all variants. A
//! variant must implement [`TTTVariantApp`] to be allowed as a variant.

use crate::{normal::NormalTTTApp, ultimate::UltimateTTTApp};
use eframe::{egui::Context, Storage};

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
                ctx.request_repaint();
                self.variant_app = Some(Box::new(NormalTTTApp::new_app(frame.storage())));
            }
        }
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        if let Some(app) = &mut self.variant_app {
            app.save_config(storage);
        }
    }
}
