//! This module provides various GUI functions that are used in multiple variants.

use crate::CellShape;
use eframe::{
    egui::Painter,
    epaint::{CircleShape, Color32, Pos2, Rect, Shape, Stroke, Vec2},
};

/// Create a centered square in the given rect, taking up the given percentage of length.
pub fn centered_square_in_rect(rect: Rect, percent: f32) -> Rect {
    let Vec2 { x, y } = rect.max - rect.min;
    let length = percent * x.min(y);

    Rect::from_center_size(rect.center(), Vec2::splat(length))
}

/// Draw the given cellshape in the given rect.
pub fn draw_cellshape_in_rect(
    painter: &Painter,
    rect: &Rect,
    shape: Option<CellShape>,
    translucent: bool,
) {
    let stroke_width = rect.width() / 30.0;

    match shape {
        None => (),
        Some(CellShape::X) => {
            let rect = centered_square_in_rect(*rect, 0.9);
            let tl = rect.min;
            let br = rect.max;
            let bl = Pos2 { x: tl.x, y: br.y };
            let tr = Pos2 { x: br.x, y: tl.y };

            let stroke = Stroke {
                width: stroke_width,
                color: if translucent {
                    let c = Color32::LIGHT_RED;
                    Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), 128)
                } else {
                    Color32::LIGHT_RED
                },
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
}

/// Draw the winning line on the board in the given rect between the given start and end coordinates.
pub fn draw_winning_line_in_rect(
    rect: &Rect,
    painter: &Painter,
    dark_mode: bool,
    start_coord: (usize, usize),
    end_coord: (usize, usize),
) {
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
            color: if dark_mode {
                Color32::WHITE
            } else {
                Color32::BLACK
            },
        },
    });
}
