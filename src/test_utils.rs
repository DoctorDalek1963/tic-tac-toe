//! This module simply contains utilities to help with unit testing.

macro_rules! mock_cell_shape {
    (X) => {
        Some($crate::shared::CellShape::X)
    };
    (O) => {
        Some($crate::shared::CellShape::O)
    };
    (E) => {
        None
    };
}

pub(crate) use mock_cell_shape;
