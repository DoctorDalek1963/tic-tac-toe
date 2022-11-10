//! This module only exists to separate the long methods used for drawing the board and cells.

use super::{send_move_when_ready, UltimateTTTApp};
use crate::{
    shared::{
        board::WinnerError,
        gui::{centered_square_in_rect, draw_cellshape_in_rect, draw_winning_line_in_rect},
    },
    ultimate::GlobalCoord,
};
use eframe::{
    egui::{self, Context, Painter, Response, Sense, Ui},
    epaint::{Color32, Pos2, Rect, Shape, Stroke, Vec2},
};

impl UltimateTTTApp {
    /// Draw board lines in the given rect with the given painter and return the width of the
    /// resultant cells.
    fn draw_board_lines(
        &self,
        ctx: &Context,
        painter: &Painter,
        rect: &Rect,
        color: Option<Color32>,
    ) -> f32 {
        let cell_length = rect.size().x / 3.0;

        let stroke = Stroke {
            width: rect.width() / 80.0,
            color: color.unwrap_or(if self.global_board.next_local_board().is_some() {
                if ctx.style().visuals.dark_mode {
                    Color32::DARK_GRAY
                } else {
                    Color32::LIGHT_GRAY
                }
            } else {
                Color32::GRAY
            }),
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

        cell_length
    }

    /// Draw the board in the given rect.
    pub fn draw_global_board(&mut self, ctx: &Context, ui: &mut Ui, rect: Rect) {
        ctx.request_repaint();

        let painter = Painter::new(
            ctx.clone(),
            egui::LayerId::new(egui::Order::Background, egui::Id::new("board_painter")),
            rect,
        );

        let cell_length = self.draw_board_lines(ctx, &painter, &rect, None);

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

                self.draw_local_board((x, y), ui, &painter, cell_rect);
            }
        }

        if self.waiting_on_move {
            if let Ok(Some(coord)) = self.mv_rx.try_recv() {
                self.update_cell(coord);
                self.waiting_on_move = false;
            }
        }

        // Draw the winning line
        if let Ok((_, [start_coord, _, end_coord])) = self.global_board.get_winner() {
            draw_winning_line_in_rect(
                &rect,
                &painter,
                ui.ctx().style().visuals.dark_mode,
                start_coord,
                end_coord,
            );
        }
    }

    /// Draw the specified local board in the given rect.
    fn draw_local_board(
        &mut self,
        coords: (usize, usize),
        ui: &mut Ui,
        painter: &Painter,
        rect: Rect,
    ) {
        let rect = centered_square_in_rect(rect, 0.85);

        let cell_length = self.draw_board_lines(
            ui.ctx(),
            painter,
            &rect,
            if let Some(c) = self.global_board.next_local_board() {
                if c == coords {
                    Some(if ui.ctx().style().visuals.dark_mode {
                        Color32::WHITE
                    } else {
                        Color32::BLACK
                    })
                } else {
                    None
                }
            } else {
                None
            },
        );

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

                let global_coord = (coords.0, coords.1, (x, y));
                if self
                    .draw_cell(ui, painter, cell_rect, global_coord)
                    .clicked()
                    && !self.waiting_on_move
                {
                    self.update_cell(global_coord);

                    if self.config.playing_ai
                        && self.global_board.get_winner() == Err(WinnerError::NoWinnerYet)
                    {
                        send_move_when_ready(
                            self.global_board.clone(),
                            self.config.max_mcts_iterations,
                            self.mv_tx.clone(),
                        );
                        self.waiting_on_move = true;
                    }
                }
            }
        }

        if let Ok((winning_shape, _)) =
            self.global_board.local_boards[coords.0][coords.1].get_winner()
        {
            draw_cellshape_in_rect(painter, &rect, Some(winning_shape), true);
        }
    }

    /// Draw the appropriate cell (specified by the [`GlobalCoord`]) in the given rect.
    fn draw_cell(
        &mut self,
        ui: &mut Ui,
        painter: &Painter,
        rect: Rect,
        coord: GlobalCoord,
    ) -> Response {
        let rect = centered_square_in_rect(rect, 0.8);
        let (x, y, (lx, ly)) = coord;
        let shape = self.global_board.local_boards[x][y].cells[lx][ly];
        let interactive: bool = (self.global_board.next_local_board() == Some((x, y))
            || self.global_board.next_local_board().is_none())
            && shape.is_none()
            && self.global_board.get_winner().is_err();

        draw_cellshape_in_rect(painter, &rect, shape, false);

        ui.allocate_rect(
            rect,
            if interactive && shape.is_none() {
                Sense::click()
            } else {
                Sense::focusable_noninteractive()
            },
        )
    }
}
