//! This module handles the `egui` interface to the game.

use crate::{
    board::{Board, CellShape},
    Coord,
};
use eframe::{
    egui::{self, Context, Painter, Rect, Response, Sense, Shape, Ui},
    epaint::{CircleShape, Color32, Pos2, Stroke, Vec2},
};
use serde::{Deserialize, Serialize};
use std::sync::mpsc;

type CoordResult = Result<Coord, ()>;

/// Create a centered square in the given rect, taking up the given percentage of length.
fn centered_square_in_rect(rect: Rect, percent: f32) -> Rect {
    let Vec2 { x, y } = rect.max - rect.min;
    let length = percent * x.min(y);

    Rect::from_center_size(rect.center(), Vec2::splat(length))
}

/// This method sends an AI-generated move down an `mpsc` channel after 200ms.
#[cfg(not(target_arch = "wasm32"))]
fn send_move_after_delay(board: Board, tx: mpsc::Sender<CoordResult>) {
    use std::{thread, time::Duration};
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(200));
        let _ = tx.send(board.generate_ai_move());
    });
}

/// This method sends an AI-generated move down an `mpsc` channel after 200ms.
#[cfg(target_arch = "wasm32")]
fn send_move_after_delay(board: Board, tx: mpsc::Sender<CoordResult>) {
    gloo_timers::callback::Timeout::new(200, move || {
        let _ = tx.send(board.generate_ai_move());
    })
    .forget();
}

/// A struct representing the app configuration, meant to be saved and loaded between sessions.
#[derive(Serialize, Deserialize)]
#[serde(default)]
struct Config {
    /// Whether the player should make the first move.
    player_plays_first: bool,

    /// Which shape the player uses.
    player_shape: CellShape,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            player_plays_first: true,
            player_shape: CellShape::X,
        }
    }
}

/// The struct to hold the state of the app.
pub struct TicTacToeApp {
    /// The configuration of the app.
    config: Config,

    /// The actual board itself.
    board: Board,

    /// The shape that will be used for the next cell to be placed.
    ///
    /// See [`update_cell`](TicTacToeApp::update_cell).
    active_shape: CellShape,

    /// Whether we're currently waiting for the AI to make a move.
    waiting_on_move: bool,

    /// The AI moves are computed in a background thread to make the UI more snappy. This is the
    /// sender that we pass to the background thread to get the AI move back.
    mv_tx: mpsc::Sender<CoordResult>,

    /// The AI moves are computed in a background thread to make the UI more snappy. This is the
    /// receiver that receives the computed AI moves.
    mv_rx: mpsc::Receiver<CoordResult>,
}

impl Default for TicTacToeApp {
    fn default() -> Self {
        Self::new_with_config(Config::default())
    }
}

impl TicTacToeApp {
    /// Create a new app, attempting to restore previous [`Config`], or using the default config.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let config = if let Some(storage) = cc.storage {
            eframe::get_value(storage, "config").unwrap_or_default()
        } else {
            Config::default()
        };

        Self::new_with_config(config)
    }

    /// Create a new app with the given config.
    ///
    /// If [`Config::player_plays_first`] is false, then we also start an AI move in the background
    /// by calling [`send_move_after_delay`].
    fn new_with_config(config: Config) -> Self {
        let (mv_tx, mv_rx) = mpsc::channel();

        let board = Board::new(config.player_shape.other());
        let waiting_on_move = !config.player_plays_first;

        let active_shape = if waiting_on_move {
            send_move_after_delay(board.clone(), mv_tx.clone());
            config.player_shape.other()
        } else {
            config.player_shape
        };

        Self {
            config,
            board,
            active_shape,
            waiting_on_move,
            mv_tx,
            mv_rx,
        }
    }

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

    fn draw_board(&mut self, ctx: &Context, ui: &mut Ui, rect: Rect) {
        ctx.request_repaint();

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
            painter.add(Shape::LineSegment {
                points: [Pos2 { x, y: rect.min.y }, Pos2 { x, y }],
                stroke,
            });

            // Draw horizontal lines
            let y = rect.min.y + (i / 3.0) * rect.height();
            let x = rect.max.x;
            painter.add(Shape::LineSegment {
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

                if Self::draw_cell(
                    ui,
                    &painter,
                    cell_rect,
                    self.board.cells[x][y],
                    self.board.get_winner().is_err(),
                )
                .clicked()
                    && !self.waiting_on_move
                {
                    self.update_cell(x, y);

                    send_move_after_delay(self.board.clone(), self.mv_tx.clone());
                    self.waiting_on_move = true;
                }
            }
        }

        if self.waiting_on_move {
            if let Ok(Ok((x, y))) = self.mv_rx.try_recv() {
                self.update_cell(x, y);
                self.waiting_on_move = false;
            }
        }

        if let Ok((_, [start_coord, _, end_coord])) = self.board.get_winner() {
            let Pos2 { x: min_x, y: min_y } = rect.min;
            let Pos2 { x: max_x, y: max_y } = rect.max;
            let len = rect.width();

            let [start, end]: [Pos2; 2] = match [start_coord, end_coord] {
                [(0, 0), (0, 2)] /* Column 0 */ => [
                    Pos2 { x: min_x + len / 6., y: min_y },
                    Pos2 { x: min_x + len / 6., y: max_y },
                ],

                [(1, 0), (1, 2)] /* Column 1 */ => [
                    Pos2 { x: min_x + len / 2., y: min_y },
                    Pos2 { x: min_x + len / 2., y: max_y },
                ],

                [(2, 0), (2, 2)] /* Column 2 */ => [
                    Pos2 { x: min_x + (5. * len / 6.), y: min_y },
                    Pos2 { x: min_x + (5. * len / 6.), y: max_y },
                ],

                [(0, 0), (2, 0)] /* Row 0 */ => [
                    Pos2 { x: min_x, y: min_y + len / 6. },
                    Pos2 { x: max_x, y: min_y + len / 6. },
                ],

                [(0, 1), (2, 1)] /* Row 1 */ => [
                    Pos2 { x: min_x, y: min_y + len / 2. },
                    Pos2 { x: max_x, y: min_y + len / 2. },
                ],

                [(0, 2), (2, 2)] /* Row 2 */ => [
                    Pos2 { x: min_x, y: min_y + (5. * len / 6.) },
                    Pos2 { x: max_x, y: min_y + (5. * len / 6.) },
                ],

                [(0, 2), (2, 0)] /* +ve diagonal */ => {
                    let x = 0.5 * len * 0.95;
                    let vec = Vec2 { x, y: -x };
                    [
                        rect.center() + vec,
                        rect.center() - vec,
                    ]
                },

                [(0, 0), (2, 2)] /* -ve diagonal */ => {
                    let vec = Vec2::splat(0.5 * len * 0.95);
                    [
                        rect.center() + vec,
                        rect.center() - vec,
                    ]
                },

                _ => unreachable!("We should have covered all possible winning lines")
            };

            let stroke_width = rect.width() / 90.0;
            painter.add(Shape::LineSegment {
                points: [start, end],
                stroke: Stroke {
                    width: stroke_width,
                    color: Color32::WHITE,
                },
            });
        }
    }

    fn draw_cell(
        ui: &mut Ui,
        painter: &Painter,
        rect: Rect,
        shape: Option<CellShape>,
        interactive: bool,
    ) -> Response {
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
                    Shape::LineSegment {
                        points: [tl, br],
                        stroke,
                    },
                    Shape::LineSegment {
                        points: [bl, tr],
                        stroke,
                    },
                ]);
            }
            Some(CellShape::O) => {
                painter.add(Shape::Circle(CircleShape {
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
                None if interactive => Sense::click(),
                _ => Sense::focusable_noninteractive(),
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

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "config", &self.config);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::make_board;
    use crate::Coord;

    #[test]
    fn update_cell_test() {
        let map_1: Vec<(Coord, Board)> = vec![
            ((0, 1), make_board!(E E E; X E E; E E E)),
            ((2, 1), make_board!(E E E; X E O; E E E)),
            ((0, 0), make_board!(X E E; X E O; E E E)),
            ((2, 2), make_board!(X E E; X E O; E E O)),
            ((2, 0), make_board!(X E X; X E O; E E O)),
            ((0, 2), make_board!(X E X; X E O; O E O)),
        ];

        let map_2: Vec<(Coord, Board)> = vec![
            ((0, 3), make_board!(E E E; E E E; E E E)),
            ((6, 3), make_board!(E E E; E E E; E E E)),
            ((1, 1), make_board!(E E E; E X E; E E E)),
            ((1, 1), make_board!(E E E; E X E; E E E)),
            ((1, 1), make_board!(E E E; E X E; E E E)),
            ((1, 1), make_board!(E E E; E X E; E E E)),
            ((2, 1), make_board!(E E E; E X O; E E E)),
        ];

        for moves_map in [map_1, map_2] {
            let mut app = TicTacToeApp::default();
            assert_eq!(app.board, Board::default());

            for ((x, y), board) in moves_map {
                app.update_cell(x, y);
                assert_eq!(app.board, board);
            }
        }
    }
}
