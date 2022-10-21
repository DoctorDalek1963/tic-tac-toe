use tictactoe::app::TicTacToeApp;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Tic-tac-toe",
        options,
        Box::new(|_cc| Box::new(TicTacToeApp::default())),
    );
}
