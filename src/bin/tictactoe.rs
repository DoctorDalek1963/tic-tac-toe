//! This crate  simply runs the tic-tac-toe GUI app.

use tictactoe::app::TTTApp;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Tic-tac-toe",
        options,
        Box::new(|cc| Box::new(TTTApp::new(cc))),
    );
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();

    let options = eframe::WebOptions::default();
    eframe::start_web(
        "main_canvas_id",
        options,
        Box::new(|cc| Box::new(TTTApp::new(cc))),
    )
    .unwrap();
}
