//! This module simply contains utilities to help with unit testing.

use crate::shared::CellShape;

pub(crate) enum MockCellShape {
    X,
    O,
    E,
}

impl Into<Option<CellShape>> for MockCellShape {
    fn into(self) -> Option<CellShape> {
        match self {
            MockCellShape::X => Some(CellShape::X),
            MockCellShape::O => Some(CellShape::O),
            MockCellShape::E => None,
        }
    }
}
