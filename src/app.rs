//! This module handles the `egui` interface to the game.

use crate::board::{Board, CellShape};
use eframe::{
    egui::{self, Context, Painter, Rect, Response, Sense, Ui},
    epaint::{CircleShape, Color32, Pos2, Stroke, Vec2},
};

/// Create a centered square in the given rect, taking up the given percentage of length.
fn centered_square_in_rect(rect: Rect, percent: f32) -> Rect {
    let Vec2 { x, y } = rect.max - rect.min;
    let length = percent * x.min(y);

    Rect::from_center_size(rect.center(), Vec2::splat(length))
}

/// The struct to hold the state of the app.
pub struct TicTacToeApp {
    /// The actual board itself.
    board: Board,

    /// The shape that will be used for the next cell to be placed.
    ///
    /// See [`update_cell`](TicTacToeApp::update_cell).
    active_shape: CellShape,
}

impl Default for TicTacToeApp {
    /// Create a default board with the player using the `X` shape.
    fn default() -> Self {
        Self::new(CellShape::X)
    }
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

    /// Create a new app with the given player shape (the player moves first).
    pub fn new(player_shape: CellShape) -> Self {
        Self {
            board: Board::new(player_shape.other()),
            active_shape: player_shape,
        }
    }

    fn draw_board(&mut self, ctx: &Context, ui: &mut Ui, rect: Rect) {
        let painter = Painter::new(
            ctx.clone(),
            egui::LayerId::new(egui::Order::Middle, egui::Id::new("board_painter")),
            rect,
        );

        let cell_length = rect.size().x / 3.0;
        let nums = [0, 1, 2];

        let stroke = Stroke {
            width: rect.width() / 80.0,
            color: Color32::GRAY,
        };
        for i in [1.0, 2.0] {
            // Draw vertical lines
            let x = rect.min.x + (i / 3.0) * rect.width();
            let y = rect.max.y;
            painter.add(egui::Shape::LineSegment {
                points: [Pos2 { x, y: rect.min.y }, Pos2 { x, y }],
                stroke,
            });

            // Draw horizontal lines
            let y = rect.min.y + (i / 3.0) * rect.height();
            let x = rect.max.x;
            painter.add(egui::Shape::LineSegment {
                points: [Pos2 { x: rect.min.x, y }, Pos2 { x, y }],
                stroke,
            });
        }

        for y in nums {
            for x in nums {
                let cell_rect = Rect::from_min_size(
                    Pos2::new(
                        rect.min.x + (x as f32 * cell_length),
                        rect.min.y + (y as f32 * cell_length),
                    ),
                    Vec2::splat(cell_length),
                );

                if Self::draw_cell(ui, &painter, cell_rect, self.board.cells[x][y]).clicked() {
                    self.update_cell(x, y);
                    if let Ok((x, y)) = self.board.generate_ai_move() {
                        self.update_cell(x, y);
                    }
                }
            }
        }
    }

    fn draw_cell(ui: &mut Ui, painter: &Painter, rect: Rect, shape: Option<CellShape>) -> Response {
        let rect = centered_square_in_rect(rect, 0.8);
        let stroke_width = rect.width() / 30.0;

        match shape {
            None => (),
            Some(CellShape::X) => {
                let rect = centered_square_in_rect(rect, 0.9);
                let tl = rect.min;
                let br = rect.max;
                let bl = Pos2 { x: tl.x, y: br.y };
                let tr = Pos2 { x: br.x, y: tl.y };

                let stroke = Stroke {
                    width: stroke_width,
                    color: Color32::LIGHT_RED,
                };

                painter.extend(vec![
                    egui::Shape::LineSegment {
                        points: [tl, br],
                        stroke,
                    },
                    egui::Shape::LineSegment {
                        points: [bl, tr],
                        stroke,
                    },
                ]);
            }
            Some(CellShape::O) => {
                painter.add(egui::Shape::Circle(CircleShape {
                    center: rect.center(),
                    radius: rect.width() / 2.2,
                    fill: Color32::TRANSPARENT,
                    stroke: Stroke {
                        width: stroke_width,
                        color: Color32::LIGHT_BLUE,
                    },
                }));
            }
        };

        ui.allocate_rect(
            rect,
            match shape {
                Some(_) => Sense::focusable_noninteractive(),
                None => Sense::click(),
            },
        )
    }
}

impl eframe::App for TicTacToeApp {
    /// Show the app itself.
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_board(ctx, ui, centered_square_in_rect(ui.clip_rect(), 0.9));
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
