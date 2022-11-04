//! This module simply contains utilities to help with unit testing.

#[rustfmt::skip]
macro_rules! mock_cell_shape {
    (X) => { Some($crate::CellShape::X) };
    (O) => { Some($crate::CellShape::O) };
    (E) => { None };
}

pub(crate) use mock_cell_shape;
