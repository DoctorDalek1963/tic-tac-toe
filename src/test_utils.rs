//! This module simply contains utilities to help with unit testing.

#[rustfmt::skip]
#[cfg_attr(feature = "bench", macro_export)]
macro_rules! mock_cell_shape {
    (X) => { Some($crate::CellShape::X) };
    (O) => { Some($crate::CellShape::O) };
    (_) => { None };
}

#[cfg(not(feature = "bench"))]
pub(crate) use mock_cell_shape;

#[cfg(feature = "bench")]
pub use mock_cell_shape;
