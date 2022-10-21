use crate::board::CellState;

pub(crate) enum MockCellState {
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
        $crate::board::Board {
            cells: [
                [
                    $crate::test_utils::MockCellState::$a.into(),
                    $crate::test_utils::MockCellState::$d.into(),
                    $crate::test_utils::MockCellState::$g.into(),
                ],
                [
                    $crate::test_utils::MockCellState::$b.into(),
                    $crate::test_utils::MockCellState::$e.into(),
                    $crate::test_utils::MockCellState::$h.into(),
                ],
                [
                    $crate::test_utils::MockCellState::$c.into(),
                    $crate::test_utils::MockCellState::$f.into(),
                    $crate::test_utils::MockCellState::$i.into(),
                ],
            ],
            ai_shape: $crate::board::CellState::O,
        }
    }};
}

pub(crate) use make_board;

#[cfg(test)]
mod tests {
    use super::*;

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
}
