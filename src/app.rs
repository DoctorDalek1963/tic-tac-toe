//! This module handles the `egui` interface to the game.

use crate::board::{Board, CellShape};
use eframe::{
    egui::{self, Rect},
    epaint::{Pos2, Vec2},
};

/// The struct to hold the state of the app.
pub struct TicTacToeApp {
    /// The actual board itself.
    board: Board,

    /// The shape that will be used for the next cell to be placed.
    ///
    /// See [`update_cell`](TicTacToeApp::update_cell).
    active_shape: CellShape,
}

impl TicTacToeApp {
    /// Update the board to reflect a cell being clicked.
    ///
    /// This method uses [`active_shape`](TicTacToeApp::active_shape) as the shape to place in the cell.
    fn update_cell(&mut self, x: usize, y: usize) {
        if x > 2 || y > 2 {
            return;
        }

        if self.board.cells[x][y].is_none() {
            self.board.cells[x][y] = Some(self.active_shape);
            self.active_shape = self.active_shape.other();
        }
    }
}

impl TicTacToeApp {
    /// Create a new app with the given player shape (the player moves first).
    pub fn new(player_shape: CellShape) -> Self {
        Self {
            board: Board::new(player_shape.other()),
            active_shape: player_shape,
        }
    }
}

impl Default for TicTacToeApp {
    /// Create a default board with the player using the `X` shape.
    fn default() -> Self {
        Self::new(CellShape::X)
    }
}

impl eframe::App for TicTacToeApp {
    /// Show the app itself.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        /// Get the string form of the given shape.
        fn get_string(x: Option<CellShape>) -> String {
            match x {
                None => "",
                Some(state) => match state {
                    CellShape::X => "X",
                    CellShape::O => "O",
                },
            }
            .to_string()
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Create a square which fills as much space as possible, and is centered
            let rect = {
                let max_len = ui.clip_rect().max;
                let length = 0.9 * max_len.x.min(max_len.y);

                Rect::from_center_size(ui.clip_rect().center(), Vec2::splat(length))
            };
            let cell_length = rect.size().x / 3.0;
            let nums = [0, 1, 2];

            for y in nums {
                for x in nums {
                    let cell_rect = Rect::from_min_size(
                        Pos2::new(
                            rect.min.x + (x as f32 * cell_length),
                            rect.min.y + (y as f32 * cell_length),
                        ),
                        Vec2::splat(cell_length),
                    );
                    let s = get_string(self.board.cells[x][y]);

                    if ui
                        .put(cell_rect, egui::Button::new(s).frame(true))
                        .clicked()
                    {
                        self.update_cell(x, y);
                        if let Ok((x, y)) = self.board.generate_ai_move() {
                            self.update_cell(x, y);
                        }
                    };
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::make_board;

    #[test]
    fn update_cell_test() {
        let map_1: Vec<((usize, usize), Board)> = vec![
            ((0, 1), make_board!(E E E; X E E; E E E)),
            ((2, 1), make_board!(E E E; X E O; E E E)),
            ((0, 0), make_board!(X E E; X E O; E E E)),
            ((2, 2), make_board!(X E E; X E O; E E O)),
            ((2, 0), make_board!(X E X; X E O; E E O)),
            ((0, 2), make_board!(X E X; X E O; O E O)),
        ];

        let map_2: Vec<((usize, usize), Board)> = vec![
            ((0, 3), make_board!(E E E; E E E; E E E)),
            ((6, 3), make_board!(E E E; E E E; E E E)),
            ((1, 1), make_board!(E E E; E X E; E E E)),
            ((1, 1), make_board!(E E E; E X E; E E E)),
            ((1, 1), make_board!(E E E; E X E; E E E)),
            ((1, 1), make_board!(E E E; E X E; E E E)),
            ((2, 1), make_board!(E E E; E X O; E E E)),
        ];

        for moves_map in [map_1, map_2] {
            let mut app = TicTacToeApp::new(CellShape::X);
            assert_eq!(app.board, Board::default());

            for ((x, y), board) in moves_map {
                app.update_cell(x, y);
                assert_eq!(app.board, board);
            }
        }
    }
}
