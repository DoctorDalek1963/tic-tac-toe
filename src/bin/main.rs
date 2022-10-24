use tictactoe::app::TicTacToeApp;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Tic-tac-toe",
        options,
        Box::new(|_cc| Box::new(TicTacToeApp::default())),
    );
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();

    let options = eframe::WebOptions::default();
    eframe::start_web(
        "main_canvas_id",
        options,
        Box::new(|_cc| Box::new(TicTacToeApp::default())),
    )
    .unwrap();
}
