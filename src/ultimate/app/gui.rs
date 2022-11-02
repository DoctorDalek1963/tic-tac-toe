//! This module only exists to separate the long methods used for drawing the board and cells.

use super::UltimateTTTApp;
use crate::shared::centered_square_in_rect;
use eframe::{
    egui::{self, Context, Painter, Ui},
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
    pub(crate) fn draw_global_board(&mut self, ctx: &Context, ui: &mut Ui, rect: Rect) {
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
    }

    /// Draw the specified local board in the given rect.
    fn draw_local_board(
        &mut self,
        coords: (usize, usize),
        ui: &mut Ui,
        painter: &Painter,
        rect: Rect,
    ) {
        self.draw_board_lines(
            ui.ctx(),
            painter,
            &centered_square_in_rect(rect, 0.85),
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
    }
}
