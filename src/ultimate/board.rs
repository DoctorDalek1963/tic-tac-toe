use crate::shared::CellShape;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LocalBoard {
    /// This 2D array represents all the cells, and is indexed as `cells[x][y]`, with the layout as so:
    ///
    /// ```text
    /// (0, 0) | (1, 0) | (2, 0)
    /// ------------------------
    /// (0, 1) | (1, 1) | (2, 1)
    /// ------------------------
    /// (0, 2) | (1, 2) | (2, 2)
    /// ```
    pub cells: [[Option<CellShape>; 3]; 3],
}

impl LocalBoard {
    pub fn new() -> Self {
        Self {
            cells: [[None; 3]; 3],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlobalBoard {
    /// This 2D array represents all the [`LocalBoard`]s, and is indexed as `cells[x][y]`, with the layout as so:
    ///
    /// ```text
    /// (0, 0) | (1, 0) | (2, 0)
    /// ------------------------
    /// (0, 1) | (1, 1) | (2, 1)
    /// ------------------------
    /// (0, 2) | (1, 2) | (2, 2)
    /// ```
    pub local_boards: [[LocalBoard; 3]; 3],

    /// This is the shape that the AI will play as.
    ///
    /// Board positions where this shape wins are considered good, and positions where the other
    /// shape wins are considered bad.
    pub ai_shape: CellShape,
}

impl Default for GlobalBoard {
    fn default() -> Self {
        Self::new(CellShape::O)
    }
}

impl GlobalBoard {
    pub fn new(ai_shape: CellShape) -> Self {
        Self {
            local_boards: [[LocalBoard::new(); 3]; 3],
            ai_shape,
        }
    }
}
