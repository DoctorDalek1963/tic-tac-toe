//! This module only exists to separate the long methods used for drawing the board and cells.

use super::{send_move_after_delay, NormalTTTApp};
use crate::shared::{centered_square_in_rect, CellShape};
use eframe::{
    egui::{self, Context, Painter, Rect, Response, Sense, Shape, Ui},
    epaint::{CircleShape, Color32, Pos2, Stroke, Vec2},
};

impl NormalTTTApp {
    /// Draw the board in the given rect.
    ///
    /// This method also handles all the updating of the internal [`Board`](crate::normal::board::Board)
    /// when cells are clicked, and triggers an AI move with [`send_move_after_delay`] if AI is enabled.
    pub(crate) fn draw_board(&mut self, ctx: &Context, ui: &mut Ui, rect: Rect) {
        ctx.request_repaint();

        let painter = Painter::new(
            ctx.clone(),
            egui::LayerId::new(egui::Order::Background, egui::Id::new("board_painter")),
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

                    if self.config.playing_ai {
                        send_move_after_delay(self.board.clone(), self.mv_tx.clone());
                        self.waiting_on_move = true;
                    }
                }
            }
        }

        if self.waiting_on_move {
            if let Ok(Some((x, y))) = self.mv_rx.try_recv() {
                self.update_cell(x, y);
                self.waiting_on_move = false;
            }
        }

        // Draw the winning line
        if let Ok((_, [start_coord, _, end_coord])) = self.board.get_winner() {
            let Pos2 { x: min_x, y: min_y } = rect.min;
            let Pos2 { x: max_x, y: max_y } = rect.max;
            let len = rect.width();

            let [start, end]: [Pos2; 2] = match [start_coord, end_coord] {
                // Column 0
                [(0, 0), (0, 2)] => [
                    Pos2 {
                        x: min_x + len / 6.,
                        y: min_y,
                    },
                    Pos2 {
                        x: min_x + len / 6.,
                        y: max_y,
                    },
                ],

                // Column 1
                [(1, 0), (1, 2)] => [
                    Pos2 {
                        x: min_x + len / 2.,
                        y: min_y,
                    },
                    Pos2 {
                        x: min_x + len / 2.,
                        y: max_y,
                    },
                ],

                // Column 2
                [(2, 0), (2, 2)] => [
                    Pos2 {
                        x: min_x + (5. * len / 6.),
                        y: min_y,
                    },
                    Pos2 {
                        x: min_x + (5. * len / 6.),
                        y: max_y,
                    },
                ],

                // Row 0
                [(0, 0), (2, 0)] => [
                    Pos2 {
                        x: min_x,
                        y: min_y + len / 6.,
                    },
                    Pos2 {
                        x: max_x,
                        y: min_y + len / 6.,
                    },
                ],

                // Row 1
                [(0, 1), (2, 1)] => [
                    Pos2 {
                        x: min_x,
                        y: min_y + len / 2.,
                    },
                    Pos2 {
                        x: max_x,
                        y: min_y + len / 2.,
                    },
                ],

                // Row 2
                [(0, 2), (2, 2)] => [
                    Pos2 {
                        x: min_x,
                        y: min_y + (5. * len / 6.),
                    },
                    Pos2 {
                        x: max_x,
                        y: min_y + (5. * len / 6.),
                    },
                ],

                // +ve diagonal
                [(0, 2), (2, 0)] => {
                    let x = 0.5 * len * 0.95;
                    let vec = Vec2 { x, y: -x };
                    [rect.center() + vec, rect.center() - vec]
                }

                // -ve diagonal
                [(0, 0), (2, 2)] => {
                    let vec = Vec2::splat(0.5 * len * 0.95);
                    [rect.center() + vec, rect.center() - vec]
                }

                _ => unreachable!("We should have covered all possible winning lines"),
            };

            let stroke_width = rect.width() / 90.0;
            painter.add(Shape::LineSegment {
                points: [start, end],
                stroke: Stroke {
                    width: stroke_width,
                    color: if ui.ctx().style().visuals.dark_mode {
                        Color32::WHITE
                    } else {
                        Color32::BLACK
                    },
                },
            });
        }
    }

    /// Draw a cell in the given rect and return a response indicated whether it was clicked.
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
