//! This module provides various types that are shared between variants.

use eframe::{
    egui::Painter,
    epaint::{CircleShape, Color32, Pos2, Rect, Shape, Stroke, Vec2},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// An enum for the shape of a cell on the board.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum CellShape {
    X,
    O,
}

impl CellShape {
    /// Return the opposite of the current shape.
    #[must_use]
    pub fn other(&self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}

/// A possible error that could occur when trying to find a winner,
#[derive(Debug, Error, PartialEq)]
pub enum WinnerError {
    /// Neither player has won, but the board is not full, so a win could occur.
    #[error("Neither player has won yet")]
    NoWinnerYet,

    /// The board is full and neither player won.
    #[error("Board is full but no-one has won")]
    BoardFullNoWinner,

    /// Both players have won.
    ///
    /// This state should never be achievable in normal play, but we need to handle the case where
    /// multiple winning triplets are found in `get_winner()` methods for variant boards.
    #[error("Both players have won")]
    MultipleWinners,
}

/// Create a centered square in the given rect, taking up the given percentage of length.
pub fn centered_square_in_rect(rect: Rect, percent: f32) -> Rect {
    let Vec2 { x, y } = rect.max - rect.min;
    let length = percent * x.min(y);

    Rect::from_center_size(rect.center(), Vec2::splat(length))
}

/// Draw the given cellshape in the given rect.
pub(crate) fn draw_cellshape_in_rect(
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
pub(crate) fn draw_winning_line_in_rect(
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

/// Check if the board is full.
///
/// This method does not check for a winner. See [`get_winner`](Board::get_winner).
pub(crate) fn is_board_full(cells: [[Option<CellShape>; 3]; 3]) -> bool {
    cells.iter().flatten().filter(|cell| cell.is_some()).count() == 9
}

/// Return the winner in the current board position, or a variant of [`WinnerError`] if there is no winner.
///
/// # Errors
///
/// - [`NoWinnerYet`](WinnerError::NoWinnerYet): There is currently no winner, but there could be
/// in the future.
/// - [`BoardFullNoWinner`](WinnerError::BoardFullNoWinner): The board is full and neither player
/// has won.
/// - [`MultipleWinners`](WinnerError::MultipleWinners): Both players have won. This should never
/// be achievable in normal play.
pub fn get_winner(
    cells: [[Option<CellShape>; 3]; 3],
) -> Result<(CellShape, [(usize, usize); 3]), WinnerError> {
    // This closure returns a tuple with the shapes and the actual coordinates
    let get_triplet =
        |coords: [(usize, usize); 3]| -> ([Option<CellShape>; 3], [(usize, usize); 3]) {
            let get_cell = |coord: (usize, usize)| -> Option<CellShape> { cells[coord.0][coord.1] };

            (
                [
                    get_cell(coords[0]),
                    get_cell(coords[1]),
                    get_cell(coords[2]),
                ],
                coords,
            )
        };

    // Each element of this array is a tuple of the shapes and the actual coordinates
    let triplets: [([Option<CellShape>; 3], [(usize, usize); 3]); 8] = [
        get_triplet([(0, 0), (0, 1), (0, 2)]), // Column 0
        get_triplet([(1, 0), (1, 1), (1, 2)]), // Column 1
        get_triplet([(2, 0), (2, 1), (2, 2)]), // Column 2
        get_triplet([(0, 0), (1, 0), (2, 0)]), // Row 0
        get_triplet([(0, 1), (1, 1), (2, 1)]), // Row 1
        get_triplet([(0, 2), (1, 2), (2, 2)]), // Row 2
        get_triplet([(0, 2), (1, 1), (2, 0)]), // +ve diagonal
        get_triplet([(0, 0), (1, 1), (2, 2)]), // -ve diagonal
    ];

    let states: Vec<(CellShape, [(usize, usize); 3])> = triplets
        .iter()
        .filter_map(
            // Map the arrays into an Option<CellShape> representing their win
            |&(shapes, coords)| match shapes {
                [Some(CellShape::X), Some(CellShape::X), Some(CellShape::X)] => {
                    Some((CellShape::X, coords))
                }
                [Some(CellShape::O), Some(CellShape::O), Some(CellShape::O)] => {
                    Some((CellShape::O, coords))
                }
                _ => None,
            },
        )
        .collect::<Vec<_>>();

    if states.len() > 1 {
        Err(WinnerError::MultipleWinners)
    } else {
        match states.get(0) {
            None => {
                if is_board_full(cells) {
                    Err(WinnerError::BoardFullNoWinner)
                } else {
                    Err(WinnerError::NoWinnerYet)
                }
            }
            Some(x) => Ok(*x),
        }
    }
}
