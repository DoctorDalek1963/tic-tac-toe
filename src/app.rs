use crate::board::{Board, CellState};
use eframe::{
    egui::{self, Rect},
    epaint::{Pos2, Vec2},
};

pub struct TicTacToeApp {
    board: Board,
    active_shape: CellState,
}

impl TicTacToeApp {
    /// Update the board to reflect a cell being clicked.
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
    pub fn new(active_shape: CellState) -> Self {
        Self {
            board: Board::default(),
            active_shape,
        }
    }
}

impl Default for TicTacToeApp {
    fn default() -> Self {
        Self::new(CellState::X)
    }
}

impl eframe::App for TicTacToeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        fn get_string(x: Option<CellState>) -> String {
            match x {
                None => "",
                Some(state) => match state {
                    CellState::X => "X",
                    CellState::O => "O",
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
                        let (x, y) = self.board.generate_ai_move();
                        self.update_cell(x, y);
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
            let mut app = TicTacToeApp::new(CellState::X);
            assert_eq!(app.board, Board::default());

            for ((x, y), board) in moves_map {
                app.update_cell(x, y);
                assert_eq!(app.board, board);
            }
        }
    }
}
