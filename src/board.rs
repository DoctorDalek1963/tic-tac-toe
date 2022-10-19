use thiserror::Error;

/// An enum for the state of a cell on the [`Board`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CellState {
    X,
    O,
}

/// A possible error that could occur when trying to find a winner,
#[derive(Debug, Error, PartialEq)]
pub enum WinnerError {
    #[error("Neither player has won yet")]
    NoWinnerYet,

    #[error("Both players have won")]
    MultipleWinners,
}

/// A struct to represent a simple tic-tac-toe board.
#[derive(Debug, PartialEq)]
pub struct Board {
    /// This 2D array represents all the cells, and is indexed as `cells[x][y]`, with the layout as so:
    ///
    /// (0, 0) | (1, 0) | (2, 0)
    /// ------------------------
    /// (0, 1) | (1, 1) | (2, 1)
    /// ------------------------
    /// (0, 2) | (1, 2) | (2, 2)
    cells: [[Option<CellState>; 3]; 3],
}

impl Board {
    /// Create a new, empty board.
    pub fn new() -> Self {
        Self {
            cells: [[None; 3]; 3],
        }
    }

    /// Return the winner in the current board position, or a variant of [`WinnerError`] if there is no winner.
    fn get_winner(&self) -> Result<CellState, WinnerError> {
        let triplets: [[Option<CellState>; 3]; 8] = [
            [self.cells[0][0], self.cells[0][1], self.cells[0][2]], // Column 0
            [self.cells[1][0], self.cells[1][1], self.cells[1][2]], // Column 1
            [self.cells[2][0], self.cells[2][1], self.cells[2][2]], // Column 2
            [self.cells[0][0], self.cells[1][0], self.cells[2][0]], // Row 0
            [self.cells[0][1], self.cells[1][1], self.cells[2][1]], // Row 1
            [self.cells[0][2], self.cells[1][2], self.cells[2][2]], // Row 2
            [self.cells[0][2], self.cells[1][1], self.cells[2][0]], // +ve diagonal
            [self.cells[0][0], self.cells[1][1], self.cells[2][2]], // -ve diagonal
        ];

        let states = triplets
            .iter()
            .filter_map(
                // Map the arrays into an Option<CellState> representing their win
                |x| match x {
                    [Some(CellState::X), Some(CellState::X), Some(CellState::X)] => {
                        Some(CellState::X)
                    }
                    [Some(CellState::O), Some(CellState::O), Some(CellState::O)] => {
                        Some(CellState::O)
                    }
                    _ => None,
                },
            )
            .collect::<Vec<_>>();

        if states.len() > 1 {
            Err(WinnerError::MultipleWinners)
        } else {
            match states.get(0) {
                None => Err(WinnerError::NoWinnerYet),
                Some(x) => Ok(*x),
            }
        }
    }

    /// Get a vector of the coords of empty cells in the board.
    ///
    /// This method searches columns before rows.
    fn empty_cells(&self) -> Vec<(usize, usize)> {
        self.cells
            .iter()
            .enumerate()
            .map(|(col, row_vec)| {
                row_vec
                    .iter()
                    .enumerate()
                    .filter_map(|(row, val)| match val {
                        None => Some((col, row)),
                        Some(_) => None,
                    })
                    .collect::<Vec<(usize, usize)>>()
            })
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[doc(hidden)]
    mod macro_utils {
        use crate::board::CellState;

        pub enum MockCellState {
            X,
            O,
            E,
        }

        impl Into<Option<CellState>> for MockCellState {
            fn into(self) -> Option<CellState> {
                match self {
                    MockCellState::X => Some(CellState::X),
                    MockCellState::O => Some(CellState::O),
                    MockCellState::E => None,
                }
            }
        }
    }

    /// Convert a series of identifiers into a board to allow for easy testing.
    ///
    /// This macro goes row-wise and separates rows with semicolons, using `E` for an empty cell.
    ///
    /// # Example
    ///
    /// The call:
    /// ```
    /// make_board!(X E O; X O E; E E E);
    /// ```
    /// would look like this:
    /// ```
    /// X| |O
    /// -----
    /// X|O|
    /// -----
    ///  | |
    /// ```
    macro_rules! make_board {
        ($a:ident $b:ident $c:ident; $d:ident $e:ident $f:ident; $g:ident $h:ident $i:ident) => {{
            use macro_utils::MockCellState;
            Board {
                cells: [
                    [
                        MockCellState::$a.into(),
                        MockCellState::$d.into(),
                        MockCellState::$g.into(),
                    ],
                    [
                        MockCellState::$b.into(),
                        MockCellState::$e.into(),
                        MockCellState::$h.into(),
                    ],
                    [
                        MockCellState::$c.into(),
                        MockCellState::$f.into(),
                        MockCellState::$i.into(),
                    ],
                ],
            }
        }};
    }

    #[test]
    fn make_board_macro_test() {
        // X|X|
        // -----
        //  |O|
        // -----
        // O| |
        let board = make_board!(X X E; E O E; O E E);
        assert_eq!(board.cells[0][0], Some(CellState::X));
        assert_eq!(board.cells[1][0], Some(CellState::X));
        assert_eq!(board.cells[2][0], None);
        assert_eq!(board.cells[0][1], None);
        assert_eq!(board.cells[1][1], Some(CellState::O));
        assert_eq!(board.cells[2][1], None);
        assert_eq!(board.cells[0][2], Some(CellState::O));
        assert_eq!(board.cells[1][2], None);
        assert_eq!(board.cells[2][2], None);

        // X| |O
        // -----
        // X|O|
        // -----
        //  | |
        let board = make_board!(X E O; X O E; E E E);
        assert_eq!(board.cells[0][0], Some(CellState::X));
        assert_eq!(board.cells[1][0], None);
        assert_eq!(board.cells[2][0], Some(CellState::O));
        assert_eq!(board.cells[0][1], Some(CellState::X));
        assert_eq!(board.cells[1][1], Some(CellState::O));
        assert_eq!(board.cells[2][1], None);
        assert_eq!(board.cells[0][2], None);
        assert_eq!(board.cells[1][2], None);
        assert_eq!(board.cells[2][2], None);

        // X|X|O
        // -----
        // O|X|X
        // -----
        // O| |O
        let board = make_board!(X X O; O X X; O E O);
        assert_eq!(board.cells[0][0], Some(CellState::X));
        assert_eq!(board.cells[1][0], Some(CellState::X));
        assert_eq!(board.cells[2][0], Some(CellState::O));
        assert_eq!(board.cells[0][1], Some(CellState::O));
        assert_eq!(board.cells[1][1], Some(CellState::X));
        assert_eq!(board.cells[2][1], Some(CellState::X));
        assert_eq!(board.cells[0][2], Some(CellState::O));
        assert_eq!(board.cells[1][2], None);
        assert_eq!(board.cells[2][2], Some(CellState::O));
    }

    #[test]
    fn get_winner_test() {
        let board = Board::new();
        assert_eq!(board.get_winner(), Err(WinnerError::NoWinnerYet));

        // X|O|X
        // -----
        //  |X|O
        // -----
        //  |O|X
        let board = make_board!(X O X; E X O; E O X);
        assert_eq!(board.get_winner(), Ok(CellState::X));

        // O|X|O
        // -----
        // X|O|X
        // -----
        // O|X|X
        let board = make_board!(O X O; X O X; O X X);
        assert_eq!(board.get_winner(), Ok(CellState::O));

        // X|O|O
        // -----
        // O|X|X
        // -----
        // X|X|O
        let board = make_board!(X O O; O X X; X X O);
        assert_eq!(board.get_winner(), Err(WinnerError::NoWinnerYet));

        // X|X|X
        // -----
        // O|O|O
        // -----
        //  | |
        let board = make_board!(X X X; O O O; E E E);
        assert_eq!(board.get_winner(), Err(WinnerError::MultipleWinners));
    }

    #[test]
    fn get_empty_cells_test() {
        let board = Board::new();
        assert_eq!(
            board.empty_cells(),
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (1, 0),
                (1, 1),
                (1, 2),
                (2, 0),
                (2, 1),
                (2, 2),
            ]
        );

        // X|O|X
        // -----
        //  |X|O
        // -----
        //  |O|X
        let board = make_board!(X O X; E X O; E O X);
        assert_eq!(board.empty_cells(), vec![(0, 1), (0, 2)]);

        // O|X|O
        // -----
        // X|O|X
        // -----
        // O|X|X
        let board = make_board!(O X O; X O X; O X X);
        assert_eq!(board.empty_cells(), vec![]);

        // X|O|O
        // -----
        // O|X|X
        // -----
        // X|X|O
        let board = make_board!(X O O; O X X; X X O);
        assert_eq!(board.empty_cells(), vec![]);

        // X|X|X
        // -----
        // O|O|O
        // -----
        //  | |
        let board = make_board!(X X X; O O O; E E E);
        assert_eq!(board.empty_cells(), vec![(0, 2), (1, 2), (2, 2)]);
    }
}
