//! This module handles the `egui` interface to the game.

mod gui;

use super::{board::GlobalBoard, GlobalCoord};
use crate::{
    app::TTTVariantApp,
    shared::{centered_square_in_rect, CellShape},
};
use eframe::egui;

/// The struct to hold the state of the app.
pub struct UltimateTTTApp {
    /// The full global board.
    global_board: GlobalBoard,

    /// The shape that will be used for the next cell to be placed.
    ///
    /// See [`update_cell`](UltimateTTTApp::update_cell).
    active_shape: CellShape,
}

impl Default for UltimateTTTApp {
    fn default() -> Self {
        Self::new(CellShape::X)
    }
}

impl UltimateTTTApp {
    /// Create a new app with the given active shape.
    pub fn new(active_shape: CellShape) -> Self {
        Self {
            global_board: GlobalBoard::new(active_shape.other()),
            active_shape,
        }
    }

    /// Update the board to reflect a cell being clicked.
    ///
    /// This method uses [`active_shape`](UltimateTTTApp::active_shape) as the shape to place in the cell.
    fn update_cell(&mut self, coord: GlobalCoord) {
        let (x, y, (lx, ly)) = coord;

        if x > 2 || y > 2 || lx > 2 || ly > 2 {
            return;
        }

        let lb = &mut self.global_board.local_boards[x][y];
        if lb.cells[lx][ly].is_none() {
            lb.cells[lx][ly] = Some(self.active_shape);
            self.active_shape = self.active_shape.other();
        }
    }
}

impl TTTVariantApp for UltimateTTTApp {
    fn new_app(_storage: Option<&dyn eframe::Storage>) -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    fn show_ui(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_global_board(ctx, ui, centered_square_in_rect(ui.clip_rect(), 0.9))
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ultimate::test_utils::make_board;

    #[test]
    fn update_cell_test() {
        let moves_map: Vec<(GlobalCoord, GlobalBoard)> = vec![
            (
                (1, 1, (1, 1)),
                make_board! {
                    () () ();
                    () (E; E X E; E) ();
                    () () ()
                },
            ),
            (
                (1, 1, (0, 0)),
                make_board! {
                    () () ();
                    () (O E E; E X E; E) ();
                    () () ()
                },
            ),
            (
                (0, 0, (0, 1)),
                make_board! {
                    (E; X E E; E) () ();
                    () (O E E; E X E; E) ();
                    () () ()
                },
            ),
            (
                (0, 1, (1, 1)),
                make_board! {
                    (E; X E E; E) () ();
                    (E; E O E; E) (O E E; E X E; E) ();
                    () () ()
                },
            ),
            (
                (1, 1, (0, 2)),
                make_board! {
                    (E; X E E; E) () ();
                    (E; E O E; E) (O E E; E X E; X E E) ();
                    () () ()
                },
            ),
        ];

        let mut app = UltimateTTTApp::default();
        assert_eq!(app.global_board, GlobalBoard::default());

        for (coord, global_board) in moves_map {
            app.update_cell(coord);
            assert_eq!(app.global_board, global_board);
        }
    }
}
